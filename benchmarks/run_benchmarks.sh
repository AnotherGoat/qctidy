#!/bin/bash
set -e

UPDATE_CHARTS=false

# Procesar argumentos
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --update-charts) UPDATE_CHARTS=true ;;
        *) echo "Parámetro desconocido: $1"; exit 1 ;;
    esac
    shift
done

echo -e "\033[36m=======================================\033[0m"
echo -e "\033[36m   Iniciando Benchmarks de QCTidy\033[0m"
echo -e "\033[36m=======================================\033[0m"

echo -e "\n\033[33m[1/3] Ejecutando cargo bench...\033[0m"
cargo bench

echo -e "\n\033[33m[2/3] Generando JSON unificado (rust_benchmark_results.json)...\033[0m"
python3 scripts/export_rust_json.py

if [ "$UPDATE_CHARTS" = true ]; then
    echo -e "\n\033[33m[3/3] Actualizando gráficos y tabla de mejoras cruzadas...\033[0m"
    python3 scripts/plot_rust_results.py
    python3 scripts/generate_speedup_table.py
else
    echo -e "\n\033[90m[3/3] Gráficos saltados (Usa --update-charts si deseas regenerarlos).\033[0m"
fi

echo -e "\n\033[32m==========================================\033[0m"
echo -e "\033[32m   ¡Todo listo y actualizado con éxito!\033[0m"
echo -e "\033[32m==========================================\033[0m"
