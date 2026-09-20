pub mod calculator;
pub mod models;
pub mod scraper;
pub mod time_estimator;

use qctidy::Graph;
use qctidy_ports::{
    EstimatedCost as PortEstimatedCost, Estimation, EstimationError, EstimatorPort,
    ProviderCostEstimates as PortProviderCostEstimates,
};

pub use calculator::calculate_costs;
pub use models::{BackendProfile, EstimatedCost, EstimationResponse, ProviderCostEstimates};
pub use scraper::get_pricing_data;
use std::io;
use std::sync::OnceLock;
pub use time_estimator::estimate_execution_time;
use tokio::runtime::Runtime;

#[cfg(test)]
mod time_estimator_tests;

#[derive(Debug, Copy, Clone)]
pub struct EstimatorAdapter;

fn shared_runtime() -> Result<&'static Runtime, EstimationError> {
    static RUNTIME: OnceLock<io::Result<Runtime>> = OnceLock::new();

    RUNTIME
        .get_or_init(Runtime::new)
        .as_ref()
        .map_err(|_error| EstimationError::RuntimeInit)
}

impl EstimatorPort for EstimatorAdapter {
    fn estimate(&self, graph: &Graph, shots: usize) -> Result<Estimation, EstimationError> {
        let profile = BackendProfile::default();
        let execution_time = estimate_execution_time(graph, shots, &profile);
        let pricing = shared_runtime()?.block_on(get_pricing_data(false));
        let costs = calculate_costs(&pricing, execution_time, shots, profile.base_time_ns)
            .into_iter()
            .map(PortProviderCostEstimates::from)
            .collect();

        Ok(Estimation {
            execution_time,
            costs,
        })
    }
}

impl From<ProviderCostEstimates> for PortProviderCostEstimates {
    fn from(estimates: ProviderCostEstimates) -> Self {
        Self {
            provider: estimates.provider,
            status: estimates.status,
            estimates: estimates
                .estimates
                .into_iter()
                .map(PortEstimatedCost::from)
                .collect(),
        }
    }
}

impl From<EstimatedCost> for PortEstimatedCost {
    fn from(cost: EstimatedCost) -> Self {
        Self {
            provider: cost.provider,
            plan_name: cost.plan_name,
            price_label: cost.price_label,
            cost_usd: cost.cost_usd,
        }
    }
}
