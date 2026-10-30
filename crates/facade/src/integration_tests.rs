use std::sync::Arc;

use crate::{
    AnalysisRequest, ApplyFixRequest, CodeGenerationRequest, ComparisonRequest, EstimationRequest,
    FixContext, FixRequest, ParsePythonRequest, ParseRequest, SerializeRequest, Session, analyze,
    apply_fix, compare, estimate, fix, generate_code, parse, parse_python, serialize,
};
use qctidy::Graph;
use qctidy_analyzer::AnalyzerAdapter;
use qctidy_codegen::CodegenAdapter;
use qctidy_converter::ConverterAdapter;
use qctidy_ports::{
    AnalysisMode, AnalysisResult, CodeGenerationTarget, ConversionFormat, DeltaAnalysisResult,
    Estimation, EstimationError, EstimatorPort, ParsedCircuit, PythonAnalysis, Scope, ScopeChild,
};
use qctidy_python_ast::PythonAstAdapter;

/// A fake estimator that never touches the network.
struct StubEstimator;

impl EstimatorPort for StubEstimator {
    fn estimate(&self, _graph: &Graph, _shots: usize) -> Result<Estimation, EstimationError> {
        Ok(Estimation {
            execution_time: 1.0,
            costs: Vec::new(),
        })
    }
}

/// Flatten the scopes of a `PythonAnalysis` into the circuits they define.
fn flatten(analysis: PythonAnalysis) -> Vec<ParsedCircuit> {
    let mut circuits = Vec::new();

    for scope in analysis.scopes {
        collect_scope(scope, &mut circuits);
    }

    circuits.extend(analysis.module_circuits);
    circuits
}

fn collect_scope(scope: Scope, circuits: &mut Vec<ParsedCircuit>) {
    for child in scope.children {
        match child {
            ScopeChild::Circuit(circuit) => circuits.push(circuit),
            ScopeChild::Scope(nested_scope) => collect_scope(nested_scope, circuits),
        }
    }
}

#[test]
fn applies_the_use_cases_to_the_vscode_test_circuit() {
    let source = include_str!("../../../vscode/test/test_file.py");
    let fix_context = FixContext::from_source(source);

    // 1. Parse the Python source.
    let python_analysis = parse_python(
        &ParsePythonRequest::new(Arc::from(source)),
        &PythonAstAdapter,
    )
    .analysis()
    .clone();

    let circuits = flatten(python_analysis);
    let checkable = circuits
        .iter()
        .filter(|circuit| circuit.build.is_some())
        .collect::<Vec<_>>();

    assert!(!checkable.is_empty(), "the file should define circuits");

    for circuit in checkable {
        let build = circuit.build.as_ref().expect("the circuit is checkable");
        let original = &build.circuit;
        let name = circuit.name.clone();

        // 2. Check for fixable patterns.
        let check = Session::default().check(original);
        assert_eq!(
            check.operation_count(),
            check.gate_count() + check.measurement_count()
        );

        // 3. Apply the fix described by every diagnostic (source edits).
        for diagnostic in check.diagnostics() {
            let request = ApplyFixRequest::new(
                Arc::from(source),
                diagnostic.clone(),
                Arc::from(build.source_map.as_slice()),
                Arc::from(name.as_str()),
            );

            // `apply_fix` must not fail; the response itself is not needed here.
            drop(
                apply_fix(&request, &fix_context, &CodegenAdapter)
                    .expect("apply_fix should succeed"),
            );
        }

        // 4. Fix the whole circuit.
        let fixed = fix(&FixRequest::new(Arc::new(original.clone()), 10)).circuit();
        Graph::from(fixed.as_ref())
            .validate()
            .expect("the fixed circuit should be valid");

        // 5. Compare the original and the fixed circuit.
        let comparison = compare(
            &ComparisonRequest::new(
                Arc::new(original.clone()),
                Arc::clone(&fixed),
                AnalysisMode::Simple,
            ),
            &AnalyzerAdapter,
        );
        assert!(matches!(
            comparison.result(),
            DeltaAnalysisResult::Simple(_)
        ));

        // 6. Analyze the fixed circuit.
        let fixed_analysis = analyze(
            &AnalysisRequest::new(Arc::clone(&fixed), AnalysisMode::Simple),
            &AnalyzerAdapter,
        );

        let AnalysisResult::Simple(metrics) = fixed_analysis.result() else {
            panic!("expected a simple analysis");
        };
        assert_eq!(
            metrics.operation_count,
            Graph::from(fixed.as_ref()).operation_count()
        );

        // 7. Generate Qiskit code for the fixed circuit.
        let code = generate_code(
            &CodeGenerationRequest::new(
                Arc::clone(&fixed),
                CodeGenerationTarget::Qiskit,
                Some(name.clone()),
            ),
            &CodegenAdapter,
        )
        .expect("generate_code should succeed");
        assert!(!code.code().is_empty());

        // 8. Serialize and parse the fixed circuit back.
        let serialized = serialize(
            &SerializeRequest::new(Arc::clone(&fixed), ConversionFormat::Json, None, None),
            &ConverterAdapter,
        )
        .expect("serialize should succeed");

        let parsed = parse(
            &ParseRequest::new(serialized.bytes(), ConversionFormat::Json),
            &ConverterAdapter,
        )
        .expect("parse should succeed");
        assert_eq!(
            parsed.circuit().operations().len(),
            fixed.operations().len()
        );

        // 9. Estimate execution time and cost.
        let estimation = estimate(
            &EstimationRequest::new(Arc::clone(&fixed), 1024),
            &StubEstimator,
        )
        .expect("estimate should succeed");
        assert!(estimation.estimation().execution_time >= 0.0);
    }
}
