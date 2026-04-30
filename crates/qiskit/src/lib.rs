mod circuit;
mod extractor;

#[pyo3::pymodule]
#[pyo3(name = "qsimplify_qiskit")]
mod bindings {
    use pyo3::prelude::*;
    use pyo3::{PyAny, PyResult};
    use qsimplify_facade::use_case;
    use qsimplify_presenter::GraphvizFormat;

    use crate::{circuit, extractor};

    #[pyclass(name = "GraphvizFormat", eq, from_py_object)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[expect(clippy::upper_case_acronyms)]
    pub(crate) enum PythonGraphvizFormat {
        GV,
        PNG,
        SVG,
    }

    impl From<PythonGraphvizFormat> for GraphvizFormat {
        fn from(value: PythonGraphvizFormat) -> Self {
            use PythonGraphvizFormat::*;

            match value {
                GV => Self::Gv,
                PNG => Self::Png,
                SVG => Self::Svg,
            }
        }
    }

    #[pyfunction]
    pub(crate) fn display_graph(circuit: &Bound<'_, PyAny>) -> PyResult<String> {
        let operations = extractor::extract_operations(circuit)?;
        Ok(use_case::display_graph(&operations))
    }

    #[pyfunction]
    pub(crate) fn display_grid(circuit: &Bound<'_, PyAny>) -> PyResult<String> {
        let operations = extractor::extract_operations(circuit)?;
        Ok(use_case::display_grid(&operations))
    }

    #[pyfunction]
    pub(crate) fn display_matrix(circuit: &Bound<'_, PyAny>) -> PyResult<String> {
        let operations = extractor::extract_operations(circuit)?;
        Ok(use_case::display_matrix(&operations))
    }

    #[pyfunction]
    pub(crate) fn to_graphviz(
        circuit: &Bound<'_, PyAny>,
        format: PythonGraphvizFormat,
    ) -> PyResult<Vec<u8>> {
        let operations = extractor::extract_operations(circuit)?;
        Ok(use_case::to_graphviz(&operations, format.into())?)
    }

    #[pyfunction]
    pub(crate) fn simplify(
        python: Python<'_>,
        circuit: &Bound<'_, PyAny>,
        iterations: u32,
    ) -> PyResult<Py<PyAny>> {
        let operations = extractor::extract_operations(circuit)?;
        let simplified = use_case::simplify(&operations, iterations);

        circuit::operations_to_circuit(python, &simplified)
    }
}
