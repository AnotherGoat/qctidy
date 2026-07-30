use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use qctidy_analyzer::AnalyzerAdapter;
use qctidy_facade::{AnalysisRequest, ComparisonRequest};
use qctidy_ports::{AnalysisMode, AnalysisResult, DeltaAnalysisResult};

use crate::extractor;

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PythonAnalysisMode>()?;
    module.add_function(wrap_pyfunction!(analyze, module)?)?;
    module.add_function(wrap_pyfunction!(compare, module)?)?;
    Ok(())
}

#[pyclass(name = "AnalysisMode", eq, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PythonAnalysisMode {
    #[pyo3(name = "SIMPLE")]
    Simple,
    #[pyo3(name = "DETAILED")]
    Detailed,
}

impl From<PythonAnalysisMode> for AnalysisMode {
    fn from(value: PythonAnalysisMode) -> Self {
        use PythonAnalysisMode::*;

        match value {
            Simple => Self::Simple,
            Detailed => Self::Detailed,
        }
    }
}

#[pyfunction(signature = (circuit, mode=PythonAnalysisMode::Simple))]
fn analyze(
    python: Python<'_>,
    circuit: &Bound<'_, PyAny>,
    mode: PythonAnalysisMode,
) -> PyResult<Py<PyAny>> {
    let extracted = extractor::extract_circuit(circuit)?;
    let request = AnalysisRequest::new(extracted.into(), mode.into());
    let response = qctidy_facade::analyze(&request, &AnalyzerAdapter)
        .map_err(|error| PyValueError::new_err(error.to_string()))?;

    match response.result() {
        AnalysisResult::Simple(metrics) => metrics_to_dict(python, metrics),
        AnalysisResult::Detailed(metrics) => detailed_metrics_to_dict(python, metrics),
    }
}

#[pyfunction(signature = (old_circuit, new_circuit, mode=PythonAnalysisMode::Simple))]
fn compare(
    python: Python<'_>,
    old_circuit: &Bound<'_, PyAny>,
    new_circuit: &Bound<'_, PyAny>,
    mode: PythonAnalysisMode,
) -> PyResult<Py<PyAny>> {
    let request = ComparisonRequest::new(
        extractor::extract_circuit(old_circuit)?.into(),
        extractor::extract_circuit(new_circuit)?.into(),
        mode.into(),
    );
    let response = qctidy_facade::compare(&request, &AnalyzerAdapter)
        .map_err(|error| PyValueError::new_err(error.to_string()))?;

    match response.result() {
        DeltaAnalysisResult::Simple(metrics) => delta_to_dict(python, &metrics),
        DeltaAnalysisResult::Detailed(metrics) => detailed_delta_to_dict(python, &metrics),
    }
}

fn metrics_to_dict(
    python: Python<'_>,
    metrics: qctidy_ports::AnalysisMetrics,
) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(python);
    dict.set_item("qubit_count", metrics.qubit_count)?;
    dict.set_item("depth", metrics.depth)?;
    dict.set_item("x_count", metrics.x_count)?;
    dict.set_item("y_count", metrics.y_count)?;
    dict.set_item("z_count", metrics.z_count)?;
    dict.set_item("pauli_count", metrics.pauli_count)?;
    dict.set_item("hadamard_count", metrics.hadamard_count)?;
    dict.set_item("rotation_count", metrics.rotation_count)?;
    dict.set_item("square_root_count", metrics.square_root_count)?;
    dict.set_item("measure_count", metrics.measure_count)?;
    dict.set_item("swap_count", metrics.swap_count)?;
    dict.set_item("cx_count", metrics.cx_count)?;
    dict.set_item("gate_count", metrics.gate_count)?;
    dict.set_item("single_gate_count", metrics.single_gate_count)?;
    dict.set_item("controlled_gate_count", metrics.controlled_gate_count)?;
    dict.set_item("auxiliary_qubit_count", metrics.auxiliary_qubit_count)?;
    dict.set_item("gate_types_count", metrics.gate_types_count)?;
    Ok(dict.into())
}

fn detailed_metrics_to_dict(
    python: Python<'_>,
    metrics: qctidy_ports::DetailedAnalysisMetrics,
) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(python);
    dict.set_item("width", metrics.width)?;
    dict.set_item("depth", metrics.depth)?;
    dict.set_item("max_density", metrics.max_density)?;
    dict.set_item("average_density", metrics.average_density)?;
    dict.set_item("x_count", metrics.x_count)?;
    dict.set_item("y_count", metrics.y_count)?;
    dict.set_item("z_count", metrics.z_count)?;
    dict.set_item("pauli_count", metrics.pauli_count)?;
    dict.set_item("hadamard_count", metrics.hadamard_count)?;
    dict.set_item(
        "initial_superposition_percent",
        metrics.initial_superposition_percent,
    )?;
    dict.set_item("other_single_qubit_count", metrics.other_single_qubit_count)?;
    dict.set_item("single_qubit_count", metrics.single_qubit_count)?;
    dict.set_item(
        "single_controlled_qubit_count",
        metrics.single_controlled_qubit_count,
    )?;
    dict.set_item("swap_count", metrics.swap_count)?;
    dict.set_item("cnot_count", metrics.cnot_count)?;
    dict.set_item("cnot_qubit_percent", metrics.cnot_qubit_percent)?;
    dict.set_item("average_cnot", metrics.average_cnot)?;
    dict.set_item("max_cnot", metrics.max_cnot)?;
    dict.set_item("toffoli_count", metrics.toffoli_count)?;
    dict.set_item("toffoli_qubit_percent", metrics.toffoli_qubit_percent)?;
    dict.set_item("average_toffoli", metrics.average_toffoli)?;
    dict.set_item("max_toffoli", metrics.max_toffoli)?;
    dict.set_item("gate_count", metrics.gate_count)?;
    dict.set_item("controlled_gate_count", metrics.controlled_gate_count)?;
    dict.set_item("single_qubit_percent", metrics.single_qubit_percent)?;
    dict.set_item("measure_count", metrics.measure_count)?;
    dict.set_item("measure_percent", metrics.measure_percent)?;
    dict.set_item("auxiliary_percent", metrics.auxiliary_percent)?;
    Ok(dict.into())
}

fn delta_to_dict(
    python: Python<'_>,
    metrics: &qctidy_ports::DeltaAnalysisMetrics,
) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(python);
    dict.set_item("qubit_count", metrics.qubit_count)?;
    dict.set_item("depth", metrics.depth)?;
    dict.set_item("x_count", metrics.x_count)?;
    dict.set_item("y_count", metrics.y_count)?;
    dict.set_item("z_count", metrics.z_count)?;
    dict.set_item("pauli_count", metrics.pauli_count)?;
    dict.set_item("hadamard_count", metrics.hadamard_count)?;
    dict.set_item("rotation_count", metrics.rotation_count)?;
    dict.set_item("square_root_count", metrics.square_root_count)?;
    dict.set_item("measure_count", metrics.measure_count)?;
    dict.set_item("swap_count", metrics.swap_count)?;
    dict.set_item("cx_count", metrics.cx_count)?;
    dict.set_item("gate_count", metrics.gate_count)?;
    dict.set_item("single_gate_count", metrics.single_gate_count)?;
    dict.set_item("controlled_gate_count", metrics.controlled_gate_count)?;
    dict.set_item("auxiliary_qubit_count", metrics.auxiliary_qubit_count)?;
    dict.set_item("gate_types_count", metrics.gate_types_count)?;
    Ok(dict.into())
}

fn detailed_delta_to_dict(
    python: Python<'_>,
    metrics: &qctidy_ports::DeltaDetailedAnalysisMetrics,
) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(python);
    dict.set_item("width", metrics.width)?;
    dict.set_item("depth", metrics.depth)?;
    dict.set_item("max_density", metrics.max_density)?;
    dict.set_item("average_density", metrics.average_density)?;
    dict.set_item("x_count", metrics.x_count)?;
    dict.set_item("y_count", metrics.y_count)?;
    dict.set_item("z_count", metrics.z_count)?;
    dict.set_item("pauli_count", metrics.pauli_count)?;
    dict.set_item("hadamard_count", metrics.hadamard_count)?;
    dict.set_item(
        "initial_superposition_percent",
        metrics.initial_superposition_percent,
    )?;
    dict.set_item("other_single_qubit_count", metrics.other_single_qubit_count)?;
    dict.set_item("single_qubit_count", metrics.single_qubit_count)?;
    dict.set_item(
        "single_controlled_qubit_count",
        metrics.single_controlled_qubit_count,
    )?;
    dict.set_item("swap_count", metrics.swap_count)?;
    dict.set_item("cnot_count", metrics.cnot_count)?;
    dict.set_item("cnot_qubit_percent", metrics.cnot_qubit_percent)?;
    dict.set_item("average_cnot", metrics.average_cnot)?;
    dict.set_item("max_cnot", metrics.max_cnot)?;
    dict.set_item("toffoli_count", metrics.toffoli_count)?;
    dict.set_item("toffoli_qubit_percent", metrics.toffoli_qubit_percent)?;
    dict.set_item("average_toffoli", metrics.average_toffoli)?;
    dict.set_item("max_toffoli", metrics.max_toffoli)?;
    dict.set_item("gate_count", metrics.gate_count)?;
    dict.set_item("controlled_gate_count", metrics.controlled_gate_count)?;
    dict.set_item("single_qubit_percent", metrics.single_qubit_percent)?;
    dict.set_item("measure_count", metrics.measure_count)?;
    dict.set_item("measure_percent", metrics.measure_percent)?;
    dict.set_item("auxiliary_percent", metrics.auxiliary_percent)?;
    Ok(dict.into())
}
