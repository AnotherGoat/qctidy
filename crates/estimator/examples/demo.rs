use qctidy::GraphBuilder;
use qctidy_estimator::{
    BackendProfile, calculate_costs, estimate_execution_time, get_pricing_data,
};

#[tokio::main]
async fn main() {
    println!("=== 1. Building a Quantum Circuit (DAG) ===");
    let mut builder = GraphBuilder::new(3);

    builder.push_h(0);
    builder.push_h(1);

    builder.push_cx(0, 1).unwrap();
    builder.push_ccx(1, 2, 0).unwrap();

    let graph = builder.build();
    println!(
        "Circuit built: {} qubits high and {} levels of depth.",
        graph.height(),
        graph.width()
    );

    println!("\n=== 2. Simulating Execution Times (Critical Path) ===");
    let profile = BackendProfile::default();
    let shots = 1000;

    let heuristic_time = estimate_execution_time(&graph, shots, &profile);
    println!(
        "The abstract heuristic time is: {} (using the default BackendProfile)",
        heuristic_time
    );
    println!("Number of shots: {}", shots);
    println!(
        "Equivalent to: {:.6} real seconds.",
        (heuristic_time * profile.base_time_ns) / 1_000_000_000.0
    );

    println!("\n=== 3. Fetching Prices from the Cloud (AWS/IBM) ===");
    println!("Fetching catalogs from the AWS Price API and the IBM web scraper...");
    let pricing_data = get_pricing_data(false).await;
    println!("Prices fetched successfully!");

    println!("\n=== 4. Calculating the Financial Budget ===");
    let costs = calculate_costs(&pricing_data, heuristic_time, shots, profile.base_time_ns);

    for provider_estimate in costs {
        println!(
            "\n>> Provider: {} (Status: {})",
            provider_estimate.provider, provider_estimate.status
        );
        if provider_estimate.status == "success" {
            for (index, plan) in provider_estimate.estimates.iter().take(3).enumerate() {
                println!("  {}) Family/Plan: {}", index + 1, plan.plan_name);
                println!("     Price label: {}", plan.price_label);
                if let Some(cost_usd) = plan.cost_usd {
                    println!("     Estimated cost for your circuit: ${cost_usd:.6} USD");
                } else {
                    println!("     Estimated cost for your circuit: N/A (request a quote)");
                }
            }
            if provider_estimate.estimates.len() > 3 {
                println!(
                    "  ... (and {} more hardware/plans)",
                    provider_estimate.estimates.len() - 3
                );
            }
        }
    }
}
