use std::io;

use qsimplify::{Graph, dto::GateOperation};

pub trait PresenterPort {
    fn present(
        &self,
        graph: &Graph,
        format: PresentationFormat,
        dpi: Option<u32>,
    ) -> Result<Vec<u8>, io::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationFormat {
    GraphvizGv,
    GraphvizPng,
    GraphvizSvg,
}

pub trait AnalyzerPort {
    fn analyze(&self, graph: &Graph) -> ();
}

pub trait CodegenPort {
    fn generate(&self, graph: &Graph, target: CodeGenerationTarget) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeGenerationTarget {
    Qiskit,
}

pub trait ConverterPort {
    fn parse(&self, input: &[u8], format: ConversionFormat) -> Vec<GateOperation>;

    fn serialize(&self, graph: &Graph, format: ConversionFormat) -> Vec<u8>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionFormat {
    Json,
    Binary,
    Base64,
}

pub trait EstimatorPort {
    fn estimate(&self, graph: &Graph) -> (f64, f64);
}
