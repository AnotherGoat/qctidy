import json
import os

import pandas

# Get the path of this script's directory
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

# 1. Read Python results
python_results = {}
python_results_path = os.path.join(SCRIPT_DIR, '..', '..', '..', 'qctidy', 'benchmarks', 'benchmark_results.json')

if os.path.exists(python_results_path):
    with open(python_results_path, 'r') as f:
        py_data = json.load(f)
    for b in py_data['benchmarks']:
        name = b['name'].split('[')[0].replace('test_', '').replace('_', ' ').title()
        if name == "Cnot Cascade":
            name = "cnot cascade"
        else:
            name = name.lower()

        qubits = int(b['param'])
        time_seconds = b['stats']['mean']

        if name not in python_results:
            python_results[name] = {}
        python_results[name][qubits] = time_seconds

# 2. Read Rust results (Criterion)
rust_results = {}
criterion_dir = os.path.join(SCRIPT_DIR, '..', '..', 'target', 'criterion')

if os.path.exists(criterion_dir):
    for algo_name in os.listdir(criterion_dir):
        algo_path = os.path.join(criterion_dir, algo_name)
        if not os.path.isdir(algo_path) or algo_name == 'report':
            continue

        algo_name_lower = algo_name.lower()
        rust_results[algo_name_lower] = {}

        for q_dir in os.listdir(algo_path):
            q_path = os.path.join(algo_path, q_dir)
            if not os.path.isdir(q_path):
                continue

            estimates_file = os.path.join(q_path, 'new', 'estimates.json')
            if os.path.exists(estimates_file):
                with open(estimates_file, 'r') as f:
                    est_data = json.load(f)
                    time_ns = est_data['mean']['point_estimate']
                    time_seconds = time_ns / 1e9

                    try:
                        qubits = int(q_dir)
                        rust_results[algo_name_lower][qubits] = time_seconds
                    except ValueError:
                        pass

# 3. Calculate speedup factor and build table
data = []
algos = list(set(list(python_results.keys()) + list(rust_results.keys())))
algos.sort()

for algo in algos:
    for q in [1, 2, 4, 8]:
        py_time = python_results.get(algo, {}).get(q, None)
        rs_time = rust_results.get(algo, {}).get(q, None)

        if py_time is not None and rs_time is not None:
            speedup = py_time / rs_time
            py_str = f"{py_time:.4f} s"
            rs_str = f"{rs_time:.6f} s"
            speedup_str = f"{speedup:,.1f}x"

            data.append({
                "Algorithm": algo.title().replace('Cnot', 'CNOT'),
                "Qubits": q,
                "Python": py_str,
                "Rust": rs_str,
                "Speedup": speedup_str
            })

df = pandas.DataFrame(data)

# Save to CSV
os.makedirs(os.path.join(SCRIPT_DIR, '..', 'results'), exist_ok=True)
csv_path = os.path.join(SCRIPT_DIR, '..', 'results', 'speedup_table.csv')
df.to_csv(csv_path, index=False)

# Save to Markdown manually
md_path = os.path.join(SCRIPT_DIR, '..', 'results', 'speedup_table.md')
with open(md_path, 'w', encoding='utf-8') as f:
    f.write("| Algorithm | Qubits | Python | Rust | Speedup |\n")
    f.write("|-----------|--------|--------|------|---------|\n")
    for row in data:
        f.write(f"| {row['Algorithm']} | {row['Qubits']} | {row['Python']} | {row['Rust']} | {row['Speedup']} |\n")

print(f"Speedup table generated at {md_path} and {csv_path}")
