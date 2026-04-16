#![warn(missing_docs)]

mod bindings;
mod extractor;

use pyo3::prelude::*;

#[pyo3::pymodule]
fn qsimplify_qiskit(_python: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
    bindings::register(module)
}
