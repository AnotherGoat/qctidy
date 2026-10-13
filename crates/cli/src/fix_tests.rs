use qctidy::{Circuit, GateOperation};
use qctidy_facade::Session;
use qctidy_ports::SourceLocation;

use crate::fix;

fn diagnostic_for(circuit: &Circuit, code: &str) -> qctidy_facade::Diagnostic {
    Session::default()
        .check(circuit)
        .diagnostics()
        .iter()
        .find(|diagnostic| *diagnostic.metadata().code() == code)
        .cloned()
        .expect("the rule should be detected")
}

#[test]
fn fix_replaces_double_s_with_a_z() {
    let circuit = Circuit::from_operations(vec![GateOperation::s(0), GateOperation::s(0)]);
    let diagnostic = diagnostic_for(&circuit, "P001");
    let source_map = vec![
        SourceLocation {
            index: 0,
            line: 0,
            column: 0,
            end_line: 0,
            end_column: 7,
        },
        SourceLocation {
            index: 1,
            line: 1,
            column: 0,
            end_line: 1,
            end_column: 7,
        },
    ];

    let source = "qc.s(0)\nqc.s(0)\n";
    let edits = fix::edits_for(source, &circuit, &diagnostic, &source_map, "qc")
        .expect("P001 should have a source fix");

    assert_eq!(fix::apply_edits(source, &edits), "qc.z(0)\n");
}

#[test]
fn fix_removes_consecutive_hadamards() {
    let circuit = Circuit::from_operations(vec![GateOperation::h(0), GateOperation::h(0)]);
    let diagnostic = diagnostic_for(&circuit, "R002");
    let source_map = vec![
        SourceLocation {
            index: 0,
            line: 0,
            column: 0,
            end_line: 0,
            end_column: 7,
        },
        SourceLocation {
            index: 1,
            line: 1,
            column: 0,
            end_line: 1,
            end_column: 7,
        },
    ];

    let source = "qc.h(0)\nqc.h(0)\n";
    let edits = fix::edits_for(source, &circuit, &diagnostic, &source_map, "qc")
        .expect("R002 should have a source fix");

    assert_eq!(fix::apply_edits(source, &edits), "\n");
}

#[test]
fn canonicalization_rules_have_no_fix() {
    let circuit = Circuit::from_operations(vec![GateOperation::x(0), GateOperation::x(2)]);
    let session = Session::default();
    let diagnostic = session
        .check(&circuit)
        .diagnostics()
        .iter()
        .find(|diagnostic| diagnostic.metadata().category().code() == "C")
        .cloned()
        .expect("a canonicalization diagnostic should be detected");

    let source_map = vec![SourceLocation {
        index: 0,
        line: 0,
        column: 0,
        end_line: 0,
        end_column: 7,
    }];

    assert!(fix::edits_for("", &circuit, &diagnostic, &source_map, "qc").is_none());
}

#[test]
fn fix_indents_multi_gate_replacements() {
    let circuit = Circuit::from_operations(vec![
        GateOperation::try_cx(1, 2).expect("distinct qubits"),
        GateOperation::try_cx(0, 1).expect("distinct qubits"),
        GateOperation::try_cx(0, 2).expect("distinct qubits"),
    ]);
    let diagnostic = diagnostic_for(&circuit, "X003");
    let source_map = vec![
        SourceLocation {
            index: 0,
            line: 0,
            column: 4,
            end_line: 0,
            end_column: 15,
        },
        SourceLocation {
            index: 1,
            line: 1,
            column: 4,
            end_line: 1,
            end_column: 15,
        },
        SourceLocation {
            index: 2,
            line: 2,
            column: 4,
            end_line: 2,
            end_column: 15,
        },
    ];

    let source = "    qc.cx(1, 2)\n    qc.cx(0, 1)\n    qc.cx(0, 2)\n";
    let edits = fix::edits_for(source, &circuit, &diagnostic, &source_map, "qc")
        .expect("X003 should have a source fix");

    assert_eq!(
        fix::apply_edits(source, &edits),
        "    qc.cx(0, 1)\n    qc.cx(1, 2)\n"
    );
}
