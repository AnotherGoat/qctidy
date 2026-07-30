use qctidy::Graph;
use thiserror::Error;

pub trait EstimatorPort {
    fn estimate(&self, graph: &Graph, shots: usize) -> Result<Estimation, EstimationError>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Estimation {
    pub execution_time: f64,
    pub costs: Vec<ProviderCostEstimates>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProviderCostEstimates {
    pub provider: String,
    pub status: String,
    pub estimates: Vec<EstimatedCost>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EstimatedCost {
    pub provider: String,
    pub plan_name: String,
    pub price_label: String,
    pub cost_usd: Option<f64>,
}

#[derive(Debug, Clone, Copy, Error)]
pub enum EstimationError {}
