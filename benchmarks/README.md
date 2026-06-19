# QSimplify-RS Benchmarks

Este directorio contiene todo el ecosistema de pruebas de rendimiento (benchmarks) diseñado para estresar el núcleo del optimizador de circuitos cuánticos en Rust (`qsimplify-rs`) y compararlo con la implementación original en Python.

## 📂 Estructura del Directorio

El ecosistema está dividido en las siguientes subcarpetas de manera modular:

- **`benches/`**: Contiene el código fuente puro en Rust (`performance.rs`) que define y ejecuta los algoritmos de prueba (Cancellation Chain, CNOT Cascade, Irreducible Brick Wall) utilizando la librería estadísticamente rigurosa **`criterion`**.
- **`scripts/`**: Contiene scripts utilitarios escritos en Python (`export_rust_json.py`, `generate_speedup_table.py`, `plot_rust_results.py`). Se encargan de recolectar la salida cruda de Criterion, darle formato JSON y generar gráficos y tablas comparativas.
- **`results/`**: Es la carpeta de salida generada automáticamente. Aquí es donde los scripts arrojan los gráficos finales (`.png`), el JSON consolidado (`rust_benchmark_results.json`) y las tablas de mejora cruzada o *Speedup* (`.csv` y `.md`).

Además de estas carpetas, en la raíz de este directorio encontrarás dos archivos ejecutables clave:
- **`run_benchmarks.ps1`**: Es el script orquestador para Windows. Se encarga de llamar a Rust, ejecutar Python, recopilar los datos y limpiar todo con un solo comando.
- **`run_benchmarks.sh`**: Es el equivalente exacto del anterior, pero diseñado para ejecutarse en sistemas operativos Mac o Linux.

## 🤔 Contexto: ¿Por qué utilizamos scripts de Python adicionales?

A diferencia de Python (que utilizaba la librería `pytest-benchmark` para generar un único archivo JSON directamente tras correr las pruebas), la herramienta de medición estándar en Rust (**Criterion**) no exporta un solo archivo consolidado. Por defecto, Criterion genera una inmensa y compleja estructura de carpetas oculta dentro de `target/criterion/`, creando cientos de micro-archivos JSON para cada iteración y estadística de prueba de forma separada.

Para solucionar esta limitación, se implementó la **capa de automatización** (`scripts/`). Estos utilitarios navegan profundamente por la carpeta oculta de Criterion, extraen matemáticamente los valores promedio y los consolidan en un único archivo limpio (`rust_benchmark_results.json`), replicando exactamente el formato antiguo. Sin estos scripts, generar los gráficos comparativos de forma directa hubiera sido imposible.

## 🚀 ¿Cómo ejecutar las pruebas?

No necesitas ejecutar comandos de Rust o Python manualmente. Todo el flujo de recolección de datos, ejecución estadística y graficado ha sido envuelto en los ejecutables automatizados de la raíz.

### Para usuarios de Windows (PowerShell)
Abre tu terminal, ingresa a esta carpeta (`benchmarks/`) y ejecuta:
```powershell
.\run_benchmarks.ps1 -UpdateCharts
```

### Para usuarios de Mac / Linux (Bash)
Abre tu terminal, ingresa a esta carpeta (`benchmarks/`) y ejecuta:
```bash
./run_benchmarks.sh --update-charts
```

### ¿Qué hace este comando?
1. **Compila y ejecuta** internamente `cargo bench` bajo el capó.
2. **Extrae** los resultados ocultos generados por Criterion (`target/criterion`) usando los scripts de Python descritos arriba.
3. **Consolida** todo en un único archivo de resultados estructurado compatible con el formato antiguo.
4. (Opcional) Si usas el parámetro `-UpdateCharts` o `--update-charts`, regenerará instantáneamente los gráficos de barras y las tablas de Speedup en la carpeta `results/`.

*(Nota: Los scripts están diseñados en modo "escritura", lo que significa que sobreescribirán los resultados antiguos en la carpeta `results/` automáticamente sin que tengas que borrar nada).*
