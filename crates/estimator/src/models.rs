use serde::{Deserialize, Serialize};

/// Heuristic weight profile for a quantum processing unit (QPU) architecture.
/// Used to simulate the execution-time cost of logical operations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BackendProfile {
    pub phase_cost: f64,
    pub single_qubit_cost: f64,
    pub sqrt_cost: f64,
    pub two_qubit_cost: f64,
    pub three_qubit_cost: f64,
    pub swap_cost: f64,
    pub measure_cost: f64,
    pub reset_cost: f64,
    pub repetition_delay_cost: f64,
    /// Base time in nanoseconds represented by one heuristic cost unit.
    pub base_time_ns: f64,
}

impl Default for BackendProfile {
    fn default() -> Self {
        Self {
            phase_cost: 0.0,
            single_qubit_cost: 1.0,
            sqrt_cost: 2.0,
            two_qubit_cost: 15.0,
            three_qubit_cost: 80.0,
            swap_cost: 45.0,
            measure_cost: 200.0,
            reset_cost: 200.0,
            repetition_delay_cost: 100.0,
            base_time_ns: 30.0,
        }
    }
}

/// Estimated final cost for a specific provider plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimatedCost {
    pub provider: String,
    pub plan_name: String,
    pub price_label: String,
    pub cost_usd: Option<f64>,
}

/// Cost estimates grouped by top-level provider, such as AWS Braket or IBM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCostEstimates {
    pub provider: String,
    pub status: String,
    pub estimates: Vec<EstimatedCost>,
}

/// Complete estimator response with simulated heuristic time and USD cost quotes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimationResponse {
    pub execution_time: f64,
    pub costs: Vec<ProviderCostEstimates>,
}
