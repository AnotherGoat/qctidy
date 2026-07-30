use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use qctidy_converter::ConverterAdapter;
use qctidy_facade::{ParseRequest, SerializeRequest};
use qctidy_ports::ConversionFormat;

use crate::{circuit, extractor};

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PythonConversionFormat>()?;
    module.add_function(wrap_pyfunction!(parse, module)?)?;
    module.add_function(wrap_pyfunction!(serialize, module)?)?;
    Ok(())
}

#[pyclass(name = "ConversionFormat", eq, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PythonConversionFormat {
    #[pyo3(name = "JSON")]
    Json,
    #[pyo3(name = "XML")]
    Xml,
    #[pyo3(name = "MESSAGE_PACK")]
    MessagePack,
    #[pyo3(name = "CBOR")]
    Cbor,
}

impl From<PythonConversionFormat> for ConversionFormat {
    fn from(value: PythonConversionFormat) -> Self {
        use PythonConversionFormat::*;

        match value {
            Json => Self::Json,
            Xml => Self::Xml,
            MessagePack => Self::MessagePack,
            Cbor => Self::Cbor,
        }
    }
}

#[pyfunction]
fn parse(python: Python<'_>, input: &[u8], format: PythonConversionFormat) -> PyResult<Py<PyAny>> {
    let request = ParseRequest::new(input.into(), format.into());

    let response = qctidy_facade::parse(&request, &ConverterAdapter)
        .map_err(|error| PyValueError::new_err(error.to_string()))?;

    circuit::circuit_to_qiskit(python, response.circuit().as_ref())
}

#[pyfunction(signature = (circuit, format, prettify=None, indentation=None))]
fn serialize(
    circuit: &Bound<'_, PyAny>,
    format: PythonConversionFormat,
    prettify: Option<bool>,
    indentation: Option<usize>,
) -> PyResult<Vec<u8>> {
    let extracted = extractor::extract_circuit(circuit)?;
    let request = SerializeRequest::new(extracted.into(), format.into(), prettify, indentation);

    let response = qctidy_facade::serialize(&request, &ConverterAdapter)
        .map_err(|error| PyValueError::new_err(error.to_string()))?;

    Ok(response.bytes().to_vec())
}
