mod circuit;
mod extractor;

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
#[expect(clippy::upper_case_acronyms)]
enum PythonDisplayFormat {
    GRAPH,
    GRID,
    MATRIX,
    CIRCUIT,
}

impl From<PythonDisplayFormat> for DisplayFormat {
    fn from(value: PythonDisplayFormat) -> Self {
        use PythonDisplayFormat::*;

        match value {
            GRAPH => Self::Graph,
            GRID => Self::Grid,
            MATRIX => Self::Matrix,
            CIRCUIT => Self::Circuit,
        }
    }
}

#[pyclass(name = "PiFormat", eq, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::upper_case_acronyms)]
enum PythonPiFormat {
    LOWERCASE,
    UPPERCASE,
    FANCY,
}

impl From<PythonPiFormat> for PiFormat {
    fn from(value: PythonPiFormat) -> Self {
        use PythonPiFormat::*;

        match value {
            LOWERCASE => Self::Lowercase,
            UPPERCASE => Self::Uppercase,
            FANCY => Self::Fancy,
        }
    }
}

#[pyclass(name = "DiracFormat", eq, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::upper_case_acronyms)]
enum PythonDiracFormat {
    ASCII,
    FANCY,
    NONE,
}

impl From<PythonDiracFormat> for DiracFormat {
    fn from(value: PythonDiracFormat) -> Self {
        use PythonDiracFormat::*;

        match value {
            ASCII => Self::Ascii,
            FANCY => Self::Fancy,
            NONE => Self::None,
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
