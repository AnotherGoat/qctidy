use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use qsimplify_converter::ConverterAdapter;
use qsimplify_facade::{ParseRequest, SerializeRequest};
use qsimplify_ports::ConversionFormat;

use crate::{circuit, extractor};

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PythonConversionFormat>()?;
    module.add_function(wrap_pyfunction!(parse, module)?)?;
    module.add_function(wrap_pyfunction!(serialize, module)?)?;
    Ok(())
}

#[pyclass(name = "ConversionFormat", eq, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::upper_case_acronyms, non_camel_case_types)]
enum PythonConversionFormat {
    JSON,
    XML,
    MESSAGE_PACK,
}

impl From<PythonConversionFormat> for ConversionFormat {
    fn from(value: PythonConversionFormat) -> Self {
        use PythonConversionFormat::*;

        match value {
            JSON => Self::Json,
            XML => Self::Xml,
            MESSAGE_PACK => Self::MessagePack,
        }
    }
}

#[pyfunction]
fn parse(python: Python<'_>, input: &[u8], format: PythonConversionFormat) -> PyResult<Py<PyAny>> {
    let request = ParseRequest::new(input.into(), format.into());

    let response = qsimplify_facade::parse(&request, &ConverterAdapter)
        .map_err(|error| PyValueError::new_err(error.to_string()))?;

    circuit::operations_to_circuit(python, &response.operations())
}

#[pyfunction(signature = (circuit, format, prettify=None, indentation=None))]
fn serialize(
    circuit: &Bound<'_, PyAny>,
    format: PythonConversionFormat,
    prettify: Option<bool>,
    indentation: Option<usize>,
) -> PyResult<Vec<u8>> {
    let operations = extractor::extract_operations(circuit)?;
    let request = SerializeRequest::new(operations.into(), format.into(), prettify, indentation);

    let response = qsimplify_facade::serialize(&request, &ConverterAdapter)
        .map_err(|error| PyValueError::new_err(error.to_string()))?;

    Ok(response.bytes().to_vec())
}
