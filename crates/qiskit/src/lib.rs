mod circuit;
mod extractor;

#[cfg(feature = "analyzer")]
mod analyzer;

#[cfg(feature = "estimator")]
mod estimator;

#[cfg(feature = "presenter-graphviz")]
mod presenter;

#[cfg(any(
    feature = "converter-cbor",
    feature = "converter-json",
    feature = "converter-msgpack",
    feature = "converter-xml",
))]
mod converter;

#[cfg(any(feature = "codegen-qiskit", feature = "codegen-openqasm3"))]
mod codegen;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use qsimplify::{DiracFormat, PiFormat};
use qsimplify_facade::{DisplayFormat, DisplayRequest, SimplificationRequest};

#[pyclass(name = "DisplayFormat", eq, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PythonDisplayFormat {
    #[pyo3(name = "GRAPH")]
    Graph,
    #[pyo3(name = "GRID")]
    Grid,
    #[pyo3(name = "MATRIX")]
    Matrix,
    #[pyo3(name = "CIRCUIT")]
    Circuit,
}

impl From<PythonDisplayFormat> for DisplayFormat {
    fn from(value: PythonDisplayFormat) -> Self {
        use PythonDisplayFormat::*;

        match value {
            Graph => Self::Graph,
            Grid => Self::Grid,
            Matrix => Self::Matrix,
            Circuit => Self::Circuit,
        }
    }
}

#[pyclass(name = "PiFormat", eq, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PythonPiFormat {
    #[pyo3(name = "LOWERCASE")]
    Lowercase,
    #[pyo3(name = "UPPERCASE")]
    Uppercase,
    #[pyo3(name = "FANCY")]
    Fancy,
}

impl From<PythonPiFormat> for PiFormat {
    fn from(value: PythonPiFormat) -> Self {
        use PythonPiFormat::*;

        match value {
            Lowercase => Self::Lowercase,
            Uppercase => Self::Uppercase,
            Fancy => Self::Fancy,
        }
    }
}

#[pyclass(name = "DiracFormat", eq, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PythonDiracFormat {
    #[pyo3(name = "ASCII")]
    Ascii,
    #[pyo3(name = "FANCY")]
    Fancy,
    #[pyo3(name = "NONE")]
    None,
}

impl From<PythonDiracFormat> for DiracFormat {
    fn from(value: PythonDiracFormat) -> Self {
        use PythonDiracFormat::*;

        match value {
            Ascii => Self::Ascii,
            Fancy => Self::Fancy,
            None => Self::None,
        }
    }
}

#[pyfunction]
fn get_features() -> Vec<&'static str> {
    vec![
        #[cfg(feature = "analyzer")]
        "analyzer",
        #[cfg(feature = "codegen-qiskit")]
        "codegen-qiskit",
        #[cfg(feature = "codegen-openqasm3")]
        "codegen-openqasm3",
        #[cfg(feature = "converter-cbor")]
        "converter-cbor",
        #[cfg(feature = "converter-json")]
        "converter-json",
        #[cfg(feature = "converter-msgpack")]
        "converter-msgpack",
        #[cfg(feature = "converter-xml")]
        "converter-xml",
        #[cfg(feature = "presenter-graphviz")]
        "presenter-graphviz",
        #[cfg(feature = "estimator")]
        "estimator",
    ]
}

#[pyfunction(signature = (circuit, format, pi_format=None, dirac_format=None))]
fn display(
    circuit: &Bound<'_, PyAny>,
    format: PythonDisplayFormat,
    pi_format: Option<PythonPiFormat>,
    dirac_format: Option<PythonDiracFormat>,
) -> PyResult<String> {
    let extracted = extractor::extract_circuit(circuit)?;
    let request = DisplayRequest::new(
        extracted.into(),
        DisplayFormat::from(format),
        pi_format.map(PiFormat::from),
        dirac_format.map(DiracFormat::from),
    );

    let response = qsimplify_facade::display(&request)
        .map_err(|error| PyValueError::new_err(error.to_string()))?;

    Ok(response.text().to_owned())
}

#[pyfunction]
fn simplify(
    python: Python<'_>,
    circuit: &Bound<'_, PyAny>,
    iterations: u32,
) -> PyResult<Py<PyAny>> {
    let extracted = extractor::extract_circuit(circuit)?;
    let request = SimplificationRequest::new(extracted.into(), iterations);

    let response = qsimplify_facade::simplify(&request)
        .map_err(|error| PyValueError::new_err(error.to_string()))?;

    circuit::circuit_to_qiskit(python, response.circuit().as_ref())
}

#[pyo3::pymodule]
#[pyo3(name = "qsimplify_qiskit")]
fn bindings(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PythonDisplayFormat>()?;
    module.add_class::<PythonPiFormat>()?;
    module.add_class::<PythonDiracFormat>()?;

    module.add_function(wrap_pyfunction!(get_features, module)?)?;
    module.add_function(wrap_pyfunction!(display, module)?)?;
    module.add_function(wrap_pyfunction!(simplify, module)?)?;

    #[cfg(feature = "analyzer")]
    analyzer::register(module)?;

    #[cfg(feature = "estimator")]
    estimator::register(module)?;

    #[cfg(feature = "presenter-graphviz")]
    presenter::register(module)?;

    #[cfg(any(
        feature = "converter-cbor",
        feature = "converter-json",
        feature = "converter-msgpack",
        feature = "converter-xml",
    ))]
    converter::register(module)?;

    #[cfg(any(feature = "codegen-qiskit", feature = "codegen-openqasm3"))]
    codegen::register(module)?;

    Ok(())
}
