use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use qsimplify_codegen::CodegenAdapter;
use qsimplify_facade::CodeGenerationRequest;
use qsimplify_ports::CodeGenerationTarget;

use crate::extractor;

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PythonCodeGenerationTarget>()?;
    module.add_function(wrap_pyfunction!(generate_code, module)?)?;
    Ok(())
}

#[pyclass(name = "CodeGenerationTarget", eq, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PythonCodeGenerationTarget {
    #[pyo3(name = "QISKIT")]
    Qiskit,
}

impl From<PythonCodeGenerationTarget> for CodeGenerationTarget {
    fn from(value: PythonCodeGenerationTarget) -> Self {
        use PythonCodeGenerationTarget::*;

        match value {
            Qiskit => Self::Qiskit,
        }
    }
}

#[pyfunction(signature = (circuit, target, circuit_name=None))]
fn generate_code(
    python: Python<'_>,
    circuit: &Bound<'_, PyAny>,
    target: PythonCodeGenerationTarget,
    circuit_name: Option<String>,
) -> PyResult<Py<PyString>> {
    let extracted = extractor::extract_circuit(circuit)?;
    let request = CodeGenerationRequest::new(
        extracted.into(),
        CodeGenerationTarget::from(target),
        circuit_name,
    );

    let response = qsimplify_facade::generate_code(&request, &CodegenAdapter)
        .map_err(|error| PyValueError::new_err(error.to_string()))?;

    Ok(PyString::new(python, response.code()).into())
}
