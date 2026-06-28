import json
import os
import datetime

def export_rust_benchmarks_to_json():
    # Obtener la ruta del directorio de este script (benchmarks/scripts)
    SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
    criterion_dir = os.path.join(SCRIPT_DIR, '..', '..', 'target', 'criterion')
    
    output_data = {
        "datetime": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "version": "rust-criterion",
        "benchmarks": []
    }
    
    if not os.path.exists(criterion_dir):
        print(f"Error: No se encontró el directorio de criterion en {criterion_dir}")
        return

    for algo_name in os.listdir(criterion_dir):
        algo_path = os.path.join(criterion_dir, algo_name)
        if not os.path.isdir(algo_path) or algo_name == 'report':
            continue
            
        for q_dir in os.listdir(algo_path):
            q_path = os.path.join(algo_path, q_dir)
            if not os.path.isdir(q_path):
                continue
                
            try:
                qubits = int(q_dir)
            except ValueError:
                continue
                
            estimates_file = os.path.join(q_path, 'new', 'estimates.json')
            if os.path.exists(estimates_file):
                with open(estimates_file, 'r') as f:
                    est_data = json.load(f)
                    
                # Criterion outputs time in nanoseconds.
                mean_ns = est_data['mean']['point_estimate']
                mean_s = mean_ns / 1e9
                
                # Intentar leer también sample.json para obtener min y max reales si están disponibles
                min_s = mean_s
                max_s = mean_s
                sample_file = os.path.join(q_path, 'new', 'sample.json')
                if os.path.exists(sample_file):
                    with open(sample_file, 'r') as f:
                        sample_data = json.load(f)
                        # sample_data['iters'] y sample_data['times'] (tiempos totales por iteración)
                        # Dividimos el tiempo por las iteraciones para sacar el tiempo por llamada
                        if 'iters' in sample_data and 'times' in sample_data:
                            times_per_call = [t / i for t, i in zip(sample_data['times'], sample_data['iters'])]
                            min_s = min(times_per_call) / 1e9
                            max_s = max(times_per_call) / 1e9
                
                algo_title = algo_name.title().replace('Cnot', 'CNOT')
                algo_formatted = algo_name.lower().replace(' ', '_')
                
                benchmark_entry = {
                    "group": None,
                    "name": f"test_{algo_formatted}[{qubits}]",
                    "fullname": f"benches/performance.rs::test_{algo_formatted}[{qubits}]",
                    "params": {
                        "n_qubits": qubits
                    },
                    "param": str(qubits),
                    "extra_info": {
                        "language": "Rust",
                        "framework": "Criterion"
                    },
                    "stats": {
                        "min": min_s,
                        "max": max_s,
                        "mean": mean_s,
                    }
                }
                
                output_data["benchmarks"].append(benchmark_entry)

    # Escribir el archivo JSON consolidado en la carpeta benchmarks/results/
    output_dir = os.path.join(SCRIPT_DIR, '..', 'results')
    os.makedirs(output_dir, exist_ok=True)
    output_path = os.path.join(output_dir, 'rust_benchmark_results.json')
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(output_data, f, indent=4)
        
    print(f"Éxito: Archivo JSON consolidado generado en {output_path}")

if __name__ == "__main__":
    export_rust_benchmarks_to_json()
