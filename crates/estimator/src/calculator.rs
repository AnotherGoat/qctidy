use crate::models::{EstimatedCost, ProviderCostEstimates};
use serde_json::Value;

/// Combine pricing data with simulated heuristic time to calculate costs in USD.
pub fn calculate_costs(
    pricing_data: &Value,
    heuristic_time: f64,
    shots: usize,
    base_time_ns: f64,
) -> Vec<ProviderCostEstimates> {
    let mut provider_estimates = Vec::new();

    let empty = serde_json::json!({});
    let providers = pricing_data.get("providers").unwrap_or(&empty);

    if let Some(aws_data) = providers.get("aws_braket") {
        let mut aws_estimates = Vec::new();
        let status = aws_data
            .get("status")
            .and_then(|s| s.as_str())
            .unwrap_or("error");

        if status == "success" {
            if let Some(qpu_prices) = aws_data
                .get("data")
                .and_then(|d| d.get("qpu_prices"))
                .and_then(|q| q.as_array())
            {
                for item in qpu_prices {
                    let provider_name = item
                        .get("hardware_provider")
                        .and_then(|s| s.as_str())
                        .unwrap_or("-")
                        .to_string();
                    let family = item
                        .get("qpu_family")
                        .and_then(|s| s.as_str())
                        .unwrap_or("-")
                        .to_string();

                    let per_task = item.get("per_task_usd").and_then(|v| v.as_f64());
                    let per_shot = item.get("per_shot_usd").and_then(|v| v.as_f64());

                    let cost = if let (Some(t), Some(s)) = (per_task, per_shot) {
                        Some(t + (shots as f64 * s))
                    } else {
                        None
                    };

                    aws_estimates.push(EstimatedCost {
                        provider: provider_name,
                        plan_name: family,
                        price_label: "Per task + per shot".to_string(),
                        cost_usd: cost,
                    });
                }
            }
        }

        provider_estimates.push(ProviderCostEstimates {
            provider: "AWS Braket".to_string(),
            status: status.to_string(),
            estimates: aws_estimates,
        });
    }

    if let Some(ibm_data) = providers.get("ibm_quantum") {
        let mut ibm_estimates = Vec::new();
        let status = ibm_data
            .get("status")
            .and_then(|s| s.as_str())
            .unwrap_or("error");

        let real_time_seconds = (heuristic_time * base_time_ns) / 1_000_000_000.0;

        if status == "success" {
            if let Some(plans) = ibm_data
                .get("data")
                .and_then(|d| d.get("plans"))
                .and_then(|p| p.as_array())
            {
                for plan in plans {
                    let plan_name = plan
                        .get("plan")
                        .and_then(|s| s.as_str())
                        .unwrap_or("-")
                        .to_string();
                    let price_sec = plan.get("price_usd_per_second").and_then(|v| v.as_f64());
                    let label = plan
                        .get("price_label")
                        .and_then(|s| s.as_str())
                        .unwrap_or("-")
                        .to_string();

                    let cost = price_sec.map(|p| real_time_seconds * p);

                    ibm_estimates.push(EstimatedCost {
                        provider: "IBM".to_string(),
                        plan_name,
                        price_label: label,
                        cost_usd: cost,
                    });
                }
            }
        }

        provider_estimates.push(ProviderCostEstimates {
            provider: "IBM Quantum".to_string(),
            status: status.to_string(),
            estimates: ibm_estimates,
        });
    }

    provider_estimates
}
