use qsimplify::Graph;
use thiserror::Error;

pub trait EstimatorPort {
    fn estimate(&self, graph: &Graph) -> (f64, f64);
}

#[derive(Debug, Clone, Copy, Error)]
pub enum EstimationError {}
