use crate::models::{EstimatedCost, ProviderCostEstimates};
use serde_json::Value;

/// Combine pricing data with simulated heuristic time to calculate costs in USD.
pub fn calculate_costs(
    pricing_data: &Value,
    heuristic_time: f64,
    shots: usize,
    base_time_ns: f64,
) -> Vec<ProviderCostEstimates> {
    let empty = serde_json::json!({});
    let providers = pricing_data.get("providers").unwrap_or(&empty);

    let mut provider_estimates = Vec::new();

    if let Some(aws_data) = providers.get("aws_braket") {
        let status = provider_status(aws_data);
        let estimates = if status == "success" {
            aws_estimates(aws_data, shots)
        } else {
            Vec::new()
        };

        provider_estimates.push(ProviderCostEstimates {
            provider: "AWS Braket".to_string(),
            status: status.to_string(),
            estimates,
        });
    }

    if let Some(ibm_data) = providers.get("ibm_quantum") {
        let status = provider_status(ibm_data);
        let estimates = if status == "success" {
            ibm_estimates(ibm_data, heuristic_time, base_time_ns)
        } else {
            Vec::new()
        };

        provider_estimates.push(ProviderCostEstimates {
            provider: "IBM Quantum".to_string(),
            status: status.to_string(),
            estimates,
        });
    }

    provider_estimates
}

fn provider_status(provider_data: &Value) -> &str {
    provider_data
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("error")
}

fn string_field(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or("-")
        .to_string()
}

fn aws_estimates(aws_data: &Value, shots: usize) -> Vec<EstimatedCost> {
    let Some(qpu_prices) = aws_data
        .get("data")
        .and_then(|data| data.get("qpu_prices"))
        .and_then(Value::as_array)
    else {
        return Vec::new();
    };

    qpu_prices
        .iter()
        .map(|item| {
            let per_task = item.get("per_task_usd").and_then(Value::as_f64);
            let per_shot = item.get("per_shot_usd").and_then(Value::as_f64);

            EstimatedCost {
                provider: string_field(item, "hardware_provider"),
                plan_name: string_field(item, "qpu_family"),
                price_label: "Per task + per shot".to_string(),
                cost_usd: per_task
                    .zip(per_shot)
                    .map(|(task, shot)| task + shots as f64 * shot),
            }
        })
        .collect()
}

fn ibm_estimates(ibm_data: &Value, heuristic_time: f64, base_time_ns: f64) -> Vec<EstimatedCost> {
    let Some(plans) = ibm_data
        .get("data")
        .and_then(|data| data.get("plans"))
        .and_then(Value::as_array)
    else {
        return Vec::new();
    };

    let real_time_seconds = (heuristic_time * base_time_ns) / 1_000_000_000.0;

    plans
        .iter()
        .map(|plan| {
            let price_per_second = plan.get("price_usd_per_second").and_then(Value::as_f64);

            EstimatedCost {
                provider: "IBM".to_string(),
                plan_name: string_field(plan, "plan"),
                price_label: string_field(plan, "price_label"),
                cost_usd: price_per_second.map(|price| real_time_seconds * price),
            }
        })
        .collect()
}
