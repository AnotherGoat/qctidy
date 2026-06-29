use serde::{Deserialize, Serialize};

/// Perfil de pesos heurísticos que representa la arquitectura de un hardware cuántico (QPU).
/// Se utiliza para simular el costo en tiempo de las operaciones lógicas.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Tiempo base en nanosegundos al que equivalen los costos heurísticos
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

/// Representa el costo estimado final de un plan específico de un proveedor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimatedCost {
    pub provider: String,
    pub plan_name: String,
    pub price_label: String,
    pub cost_usd: Option<f64>,
}

/// Representa un reporte de costos agrupado por el proveedor principal (ej. AWS Braket, IBM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCostEstimates {
    pub provider: String,
    pub status: String,
    pub estimates: Vec<EstimatedCost>,
}

/// Respuesta completa del estimador, empaquetando el tiempo heurístico simulado
/// y las cotizaciones financieras en USD.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimationResponse {
    pub execution_time: f64,
    pub costs: Vec<ProviderCostEstimates>,
}
