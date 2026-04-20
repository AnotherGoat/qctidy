mod circuit;
mod extractor;

#[pyo3::pymodule]
#[pyo3(name = "qsimplify_qiskit")]
mod bindings {
    use pyo3::prelude::*;
    use pyo3::{PyAny, PyResult};
    use qsimplify_facade::use_case;

    use crate::{circuit, extractor};

    #[pyfunction]
    pub fn display_graph(circuit: &Bound<'_, PyAny>) -> PyResult<String> {
        let operations = extractor::extract_operations(circuit)?;
        Ok(use_case::display_graph(&operations))
    }

    #[pyfunction]
    pub fn display_grid(circuit: &Bound<'_, PyAny>) -> PyResult<String> {
        let operations = extractor::extract_operations(circuit)?;
        Ok(use_case::display_grid(&operations))
    }

    #[pyfunction]
    pub fn simplify(
        python: Python<'_>,
        circuit: &Bound<'_, PyAny>,
        iterations: u32,
    ) -> PyResult<Py<PyAny>> {
        let operations = extractor::extract_operations(circuit)?;
        let simplified = use_case::simplify(&operations, iterations);

        circuit::operations_to_circuit(python, &simplified)
    }
}
