#!/bin/bash
set -e

UPDATE_CHARTS=false

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --update-charts) UPDATE_CHARTS=true ;;
        *) echo "Unknown parameter: $1"; exit 1 ;;
    esac
    shift
done

echo -e "\033[36m=======================================\033[0m"
echo -e "\033[36m   Starting QCTidy Benchmarks\033[0m"
echo -e "\033[36m=======================================\033[0m"

echo -e "\n\033[33m[1/3] Running cargo bench...\033[0m"
cargo bench

echo -e "\n\033[33m[2/3] Generating unified JSON (rust_benchmark_results.json)...\033[0m"
python3 scripts/export_rust_json.py

if [ "$UPDATE_CHARTS" = true ]; then
    echo -e "\n\033[33m[3/3] Updating charts and cross-language speedup table...\033[0m"
    python3 scripts/plot_rust_results.py
    python3 scripts/generate_speedup_table.py
else
    echo -e "\n\033[90m[3/3] Charts skipped (use --update-charts to regenerate).\033[0m"
fi

echo -e "\n\033[32m==========================================\033[0m"
echo -e "\033[32m   All done and updated successfully!\033[0m"
echo -e "\033[32m==========================================\033[0m"
