use qsimplify::Graph;
use thiserror::Error;

pub trait AnalyzerPort {
    fn analyze(&self, graph: &Graph) -> ();
}

#[derive(Debug, Clone, Copy, Error)]
pub enum AnalysisError {}
