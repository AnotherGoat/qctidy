import json
import matplotlib.pyplot as plt
import numpy as np
import os

# Obtener la ruta del directorio de este script
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

# Leer resultados de Rust (Criterion)
rust_results = {}
criterion_dir = os.path.join(SCRIPT_DIR, '..', '..', 'target', 'criterion')

if os.path.exists(criterion_dir):
    for algo_name in os.listdir(criterion_dir):
        algo_path = os.path.join(criterion_dir, algo_name)
        if not os.path.isdir(algo_path) or algo_name == 'report':
            continue

        rust_results[algo_name] = {}

        for q_dir in os.listdir(algo_path):
            q_path = os.path.join(algo_path, q_dir)
            if not os.path.isdir(q_path):
                continue

            estimates_file = os.path.join(q_path, 'new', 'estimates.json')
            if os.path.exists(estimates_file):
                with open(estimates_file, 'r') as f:
                    est_data = json.load(f)
                    # Convertir a segundos
                    time_ns = est_data['mean']['point_estimate']
                    time_seconds = time_ns / 1e9

                    try:
                        qubits = int(q_dir)
                        rust_results[algo_name][qubits] = time_seconds
                    except ValueError:
                        pass

# Preparar datos para el gráfico de barras
labels = ['1 Qubit', '2 Qubits', '4 Qubits', '8 Qubits']
qubit_vals = [1, 2, 4, 8]

algos = list(rust_results.keys())
algos.sort()

times_per_algo = []
for algo in algos:
    times = [rust_results[algo].get(q, 0) for q in qubit_vals]
    times_per_algo.append(times)

x = np.arange(len(labels))  # Localización de las etiquetas
width = 0.25  # Ancho de las barras

fig, ax = plt.subplots(figsize=(10, 6), facecolor='white')

colors = ['#1f77b4', '#ff7f0e', '#2ca02c']

# Crear barras agrupadas
for i, algo in enumerate(algos):
    offset = width * i - width
    ax.bar(x + offset, times_per_algo[i], width, label=algo, color=colors[i], edgecolor='black')

# Añadir etiquetas y títulos
ax.set_ylabel('Tiempo en Segundos (s)', fontsize=12)
ax.set_title('Rendimiento de QCTidy en Rust', fontsize=16, pad=20)
ax.set_xticks(x)
ax.set_xticklabels(labels, fontsize=12)

# Evitar notación científica (10 elevado a x) y forzar decimales simples
ax.ticklabel_format(style='plain', axis='y')

# Leyenda fuera del gráfico
ax.legend(loc='upper center', bbox_to_anchor=(0.5, -0.12), ncol=3, fontsize=11)

# Poner cuadrícula
ax.yaxis.grid(True, linestyle='--', alpha=0.7)
ax.set_axisbelow(True)

plt.tight_layout()

os.makedirs(os.path.join(SCRIPT_DIR, '..', 'results'), exist_ok=True)
plt.savefig(os.path.join(SCRIPT_DIR, '..', 'results', 'rust_rendimiento_barras.png'), dpi=300, bbox_inches='tight')
print("Gráfico de rendimiento de Rust generado con éxito.")
