use qsimplify_estimator::{
    BackendProfile, estimate_execution_time, get_pricing_data, calculate_costs
};
use qsimplify::GraphBuilder;

#[tokio::main]
async fn main() {
    println!("=== 1. Construyendo un Circuito Cuántico (DAG) ===");
    // Circuito de 3 qubits (ej. Teleportación Cuántica simplificada)
    let mut builder = GraphBuilder::new(3);
    
    // Nivel 0: Dos operaciones Hadamard en paralelo
    builder.push_h(0);
    builder.push_h(1);
    
    // Nivel 1: Entrelazamiento (CNOT y Toffoli)
    builder.push_cx(0, 1).unwrap();
    builder.push_ccx(1, 2, 0).unwrap(); 
    
    let graph = builder.build();
    println!("Circuito construido: {} qubits de alto y {} niveles de profundidad.", graph.height(), graph.width());
    
    println!("\n=== 2. Simulando Tiempos de Ejecución (Ruta Crítica) ===");
    let profile = BackendProfile::default();
    let shots = 1000;
    
    // Simula el tiempo que tomará procesar teniendo en cuenta el paralelismo de las compuertas
    let heuristic_time = estimate_execution_time(&graph, shots, &profile);
    println!("El tiempo heurístico abstracto es: {} (usando el perfil BackendProfile por defecto)", heuristic_time);
    println!("Cantidad de disparos (Shots): {}", shots);
    println!("Equivale a: {:.6} segundos reales.", (heuristic_time * profile.base_time_ns) / 1_000_000_000.0);

    println!("\n=== 3. Extraiendo Precios de la Nube (AWS/IBM) ===");
    println!("Obteniendo catálogos desde AWS Price API y el Web Scraper de IBM...");
    // Cambiar a true si quieres forzar la descarga de internet ignorando el caché
    let pricing_data = get_pricing_data(false).await; 
    println!("¡Precios obtenidos con éxito!");

    println!("\n=== 4. Calculando Presupuesto Financiero ===");
    let costs = calculate_costs(&pricing_data, heuristic_time, shots, profile.base_time_ns);
    
    for provider_estimate in costs {
        println!("\n>> Proveedor: {} (Status: {})", provider_estimate.provider, provider_estimate.status);
        if provider_estimate.status == "success" {
            // Mostrar solo los primeros 3 planes para no saturar la terminal
            for (i, plan) in provider_estimate.estimates.iter().take(3).enumerate() {
                println!("  {}) Familia/Plan: {}", i + 1, plan.plan_name);
                println!("     Etiqueta de Precio: {}", plan.price_label);
                if let Some(cost_usd) = plan.cost_usd {
                    println!("     Costo Estimado para tu Circuito: ${:.6} USD", cost_usd);
                } else {
                    println!("     Costo Estimado para tu Circuito: N/A (Consultar)");
                }
            }
            if provider_estimate.estimates.len() > 3 {
                println!("  ... (y {} hardware/planes más)", provider_estimate.estimates.len() - 3);
            }
        }
    }
}
