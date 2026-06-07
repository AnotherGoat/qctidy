use qsimplify::Graph;
use thiserror::Error;

pub trait PresenterPort {
    fn present(
        &self,
        graph: &Graph,
        format: PresentationFormat,
        dpi: Option<u32>,
    ) -> Result<Vec<u8>, PresentationError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationFormat {
    GraphvizGv,
    GraphvizPng,
    GraphvizSvg,
}

impl PresentationFormat {
    #[must_use]
    pub const fn is_available(self) -> bool {
        use PresentationFormat::*;

        match self {
            GraphvizGv | GraphvizPng | GraphvizSvg => cfg!(feature = "presenter-graphviz"),
        }
    }
}

#[derive(Debug, Error)]
pub enum PresentationError {
    #[error("{message}")]
    CommandNotFound { message: String },
    #[error("{message}")]
    ExecutionFailed { message: String },
    #[error("{message}")]
    FileWriteFailed { message: String },
    #[error("{message}")]
    Unknown { message: String },
}
