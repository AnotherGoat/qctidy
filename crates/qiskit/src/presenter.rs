use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use qctidy_facade::PresentationRequest;
use qctidy_ports::PresentationFormat;
use qctidy_presenter::GraphvizPresenter;

use crate::extractor;

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PythonPresentationFormat>()?;
    module.add_function(wrap_pyfunction!(present, module)?)?;
    Ok(())
}

#[pyclass(name = "PresentationFormat", eq, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::enum_variant_names)]
enum PythonPresentationFormat {
    #[pyo3(name = "GRAPHVIZ_GV")]
    GraphvizGv,
    #[pyo3(name = "GRAPHVIZ_PNG")]
    GraphvizPng,
    #[pyo3(name = "GRAPHVIZ_SVG")]
    GraphvizSvg,
}

impl From<PythonPresentationFormat> for PresentationFormat {
    fn from(value: PythonPresentationFormat) -> Self {
        use PythonPresentationFormat::*;

        match value {
            GraphvizGv => Self::GraphvizGv,
            GraphvizPng => Self::GraphvizPng,
            GraphvizSvg => Self::GraphvizSvg,
        }
    }
}

#[pyfunction(signature = (circuit, format, dpi=None))]
fn present(
    circuit: &Bound<'_, PyAny>,
    format: PythonPresentationFormat,
    dpi: Option<u32>,
) -> PyResult<Vec<u8>> {
    let extracted = extractor::extract_circuit(circuit)?;
    let request = PresentationRequest::new(extracted.into(), PresentationFormat::from(format), dpi);

    let response = qctidy_facade::present(&request, &GraphvizPresenter)
        .map_err(|error| PyRuntimeError::new_err(error.to_string()))?;

    Ok(response.bytes().to_vec())
}
