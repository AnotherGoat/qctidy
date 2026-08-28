param (
    [switch]$UpdateCharts
)

Write-Host "=======================================" -ForegroundColor Cyan
Write-Host "   Starting QCTidy Benchmarks" -ForegroundColor Cyan
Write-Host "=======================================" -ForegroundColor Cyan

# 1. Run Rust performance benchmarks
Write-Host "`n[1/3] Running cargo bench..." -ForegroundColor Yellow
cargo bench

if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: Benchmarks failed." -ForegroundColor Red
    exit $LASTEXITCODE
}

# 2. Export to JSON
Write-Host "`n[2/3] Generating unified JSON (rust_benchmark_results.json)..." -ForegroundColor Yellow
python scripts/export_rust_json.py

if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: Failed to generate consolidated JSON." -ForegroundColor Red
    exit $LASTEXITCODE
}

# 3. Optional: Update charts and speedup table
if ($UpdateCharts) {
    Write-Host "`n[3/3] Updating charts and cross-language speedup table..." -ForegroundColor Yellow
    python scripts/plot_rust_results.py
    python scripts/generate_speedup_table.py
}
else {
    Write-Host "`n[3/3] Charts skipped (use -UpdateCharts to regenerate)." -ForegroundColor DarkGray
}

Write-Host "`n==========================================" -ForegroundColor Green
Write-Host "   All done and updated successfully!" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green
