mod circuit;
mod extractor;

#[pyo3::pymodule]
#[pyo3(name = "qsimplify_qiskit")]
mod bindings {
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;
    use pyo3::{PyAny, PyResult};
    use qsimplify::{DiracFormat, PiFormat};
    use qsimplify_converter::ConverterAdapter;
    use qsimplify_facade::{
        DisplayFormat, DisplayRequest, ParseRequest, PresentationRequest, SerializeRequest,
        SimplificationRequest,
    };
    use qsimplify_ports::{ConversionFormat, PresentationFormat};
    use qsimplify_presenter::GraphvizPresenter;

    use crate::{circuit, extractor};

    #[pyclass(name = "DisplayFormat", eq, from_py_object)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[expect(clippy::upper_case_acronyms)]
    pub(crate) enum PythonDisplayFormat {
        GRAPH,
        GRID,
        MATRIX,
    }

    impl From<PythonDisplayFormat> for DisplayFormat {
        fn from(value: PythonDisplayFormat) -> Self {
            use PythonDisplayFormat::*;

            match value {
                GRAPH => Self::Graph,
                GRID => Self::Grid,
                MATRIX => Self::Matrix,
            }
        }
    }

    #[pyclass(name = "PiFormat", eq, from_py_object)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[expect(clippy::upper_case_acronyms)]
    pub(crate) enum PythonPiFormat {
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
    pub(crate) enum PythonDiracFormat {
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

    #[pyclass(name = "PresentationFormat", eq, from_py_object)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[expect(non_camel_case_types)]
    pub(crate) enum PythonPresentationFormat {
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

    #[pyclass(name = "ConversionFormat", eq, from_py_object)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[expect(clippy::upper_case_acronyms, non_camel_case_types)]
    pub(crate) enum PythonConversionFormat {
        JSON,
        MESSAGE_PACK,
        BINARY,
        BASE64,
    }

    impl From<PythonConversionFormat> for ConversionFormat {
        fn from(value: PythonConversionFormat) -> Self {
            use PythonConversionFormat::*;

            match value {
                JSON => Self::Json,
                MESSAGE_PACK => Self::MessagePack,
                BINARY => Self::Binary,
                BASE64 => Self::Base64,
            }
        }
    }

    #[pyfunction(signature = (circuit, format, pi_format=None, dirac_format=None))]
    pub(crate) fn display(
        circuit: &Bound<'_, PyAny>,
        format: PythonDisplayFormat,
        pi_format: Option<PythonPiFormat>,
        dirac_format: Option<PythonDiracFormat>,
    ) -> PyResult<String> {
        let operations = extractor::extract_operations(circuit)?;
        let request = DisplayRequest::new(
            operations.into(),
            DisplayFormat::from(format),
            pi_format.map(PiFormat::from),
            dirac_format.map(DiracFormat::from),
        );

        let response = qsimplify_facade::display(&request);
        Ok(response.text().to_owned())
    }

    #[pyfunction(signature = (circuit, format, dpi=None))]
    pub(crate) fn present(
        circuit: &Bound<'_, PyAny>,
        format: PythonPresentationFormat,
        dpi: Option<u32>,
    ) -> PyResult<Vec<u8>> {
        let operations = extractor::extract_operations(circuit)?;
        let request =
            PresentationRequest::new(operations.into(), PresentationFormat::from(format), dpi);

        let response = qsimplify_facade::present(&request, &GraphvizPresenter)?;
        Ok(response.bytes().to_vec())
    }

    #[pyfunction]
    pub(crate) fn simplify(
        python: Python<'_>,
        circuit: &Bound<'_, PyAny>,
        iterations: u32,
    ) -> PyResult<Py<PyAny>> {
        let operations = extractor::extract_operations(circuit)?;
        let request = SimplificationRequest::new(operations.into(), iterations);

        let response = qsimplify_facade::simplify(&request);
        circuit::operations_to_circuit(python, &response.operations())
    }

    #[pyfunction]
    pub(crate) fn parse(
        python: Python<'_>,
        input: &[u8],
        format: PythonConversionFormat,
    ) -> PyResult<Py<PyAny>> {
        let request = ParseRequest::new(input.into(), format.into());

        let response = qsimplify_facade::parse(&request, &ConverterAdapter)
            .map_err(|error| PyValueError::new_err(error.to_string()))?;

        circuit::operations_to_circuit(python, &response.operations())
    }

    #[pyfunction(signature = (circuit, format, prettify=None, indentation=None))]
    pub(crate) fn serialize(
        circuit: &Bound<'_, PyAny>,
        format: PythonConversionFormat,
        prettify: Option<bool>,
        indentation: Option<usize>,
    ) -> PyResult<Vec<u8>> {
        let operations = extractor::extract_operations(circuit)?;
        let request =
            SerializeRequest::new(operations.into(), format.into(), prettify, indentation);

        let response = qsimplify_facade::serialize(&request, &ConverterAdapter)
            .map_err(|error| PyValueError::new_err(error.to_string()))?;

        Ok(response.bytes().to_vec())
    }
}
