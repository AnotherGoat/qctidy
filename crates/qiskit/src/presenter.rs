use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use qsimplify_facade::PresentationRequest;
use qsimplify_ports::PresentationFormat;
use qsimplify_presenter::GraphvizPresenter;

use crate::extractor;

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PythonPresentationFormat>()?;
    module.add_function(wrap_pyfunction!(present, module)?)?;
    Ok(())
}

#[pyclass(name = "PresentationFormat", eq, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(non_camel_case_types)]
enum PythonPresentationFormat {
    GRAPHVIZ_GV,
    GRAPHVIZ_PNG,
    GRAPHVIZ_SVG,
}

impl From<PythonPresentationFormat> for PresentationFormat {
    fn from(value: PythonPresentationFormat) -> Self {
        use PythonPresentationFormat::*;

        match value {
            GRAPHVIZ_GV => Self::GraphvizGv,
            GRAPHVIZ_PNG => Self::GraphvizPng,
            GRAPHVIZ_SVG => Self::GraphvizSvg,
        }
    }
}

#[pyfunction(signature = (circuit, format, dpi=None))]
fn present(
    circuit: &Bound<'_, PyAny>,
    format: PythonPresentationFormat,
    dpi: Option<u32>,
) -> PyResult<Vec<u8>> {
    let operations = extractor::extract_operations(circuit)?;
    let request =
        PresentationRequest::new(operations.into(), PresentationFormat::from(format), dpi);

    let response = qsimplify_facade::present(&request, &GraphvizPresenter)
        .map_err(|error| PyRuntimeError::new_err(error.to_string()))?;

    Ok(response.bytes().to_vec())
}
