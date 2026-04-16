use pyo3::prelude::*;
use pyo3::{PyAny, PyResult};
use qsimplify_facade::mapper;

use crate::extractor;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(build_graph, module)?)?;
    Ok(())
}

#[pyfunction]
pub fn build_graph(circuit: &Bound<'_, PyAny>) -> PyResult<String> {
    let operations = extractor::extract_operations(circuit)?;
    let graph = mapper::build_graph_from_operations(operations);
    let result = graph.display_grid();

    Ok(result)
}
