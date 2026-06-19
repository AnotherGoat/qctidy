param (
    [switch]$UpdateCharts
)

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "   Iniciando Benchmarks de QSimplify-RS" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

# 1. Ejecutar las pruebas de rendimiento en Rust (desde adentro de la carpeta)
Write-Host "`n[1/3] Ejecutando cargo bench..." -ForegroundColor Yellow
cargo bench

if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: Las pruebas fallaron." -ForegroundColor Red
    exit $LASTEXITCODE
}

# 2. Exportar a JSON
Write-Host "`n[2/3] Generando JSON unificado (rust_benchmark_results.json)..." -ForegroundColor Yellow
python scripts/export_rust_json.py

if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: Falló la generación del JSON consolidado." -ForegroundColor Red
    exit $LASTEXITCODE
}

# 3. Opcional: Actualizar gráficos y tabla de Speedup
if ($UpdateCharts) {
    Write-Host "`n[3/3] Actualizando gráficos y tabla de mejoras cruzadas..." -ForegroundColor Yellow
    python scripts/plot_rust_results.py
    python scripts/generate_speedup_table.py
}
else {
    Write-Host "`n[3/3] Gráficos saltados (Usa -UpdateCharts si deseas regenerarlos)." -ForegroundColor DarkGray
}

Write-Host "`n==========================================" -ForegroundColor Green
Write-Host "   ¡Todo listo y actualizado con éxito!" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green
