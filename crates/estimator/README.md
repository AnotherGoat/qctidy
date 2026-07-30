# `qctidy-estimator`

El módulo `qctidy-estimator` es una biblioteca nativa en Rust que proporciona herramientas avanzadas para predecir el **tiempo de ejecución físico** y el **costo financiero en dólares (USD)** de los circuitos cuánticos (`Graph`) en proveedores reales de hardware cuántico, específicamente **AWS Braket** e **IBM Quantum**.

Este módulo fue diseñado como un reemplazo directo y optimizado (port) de scripts de Python, llevando toda la recolección de datos y la simulación heurística al ecosistema seguro y concurrente de Rust.

---

## Características Principales

- **Scraper Nativo (Web + API)**: Extrae de forma automática y asíncrona los precios actuales de internet. Utiliza la API de AWS Price List para Braket y parsea el catálogo de precios de IBM Quantum usando `reqwest` y `scraper`.
- **Caché Inteligente**: Guarda los precios en un archivo local (`pricing_cache.json`) válido por 24 horas para no saturar las redes y acelerar dramáticamente el cálculo en ejecuciones subsecuentes.
- **Motor Heurístico de Tiempo (DAG)**: Utiliza un algoritmo de **Ruta Crítica (Critical Path)** sobre el Grafo Acíclico Dirigido del circuito para estimar el tiempo, logrando calcular el grado de paralelismo con exactitud.
- **Calculadora Financiera**: Cruza el tiempo heurístico del DAG y los datos del scraper para generar presupuestos exactos según el modelo de facturación de cada proveedor.

---

## ¿Cómo funciona la estimación de tiempo?

A diferencia de un simple conteo de compuertas, el `time_estimator` simula el reloj físico de cada qubit de manera independiente respetando la sincronización:

1. **Pesos Heurísticos (`BackendProfile`)**: A cada compuerta se le asigna un "peso" abstracto de tiempo. Por ejemplo, compuertas de 1 qubit cuestan `1.0`, y compuertas de 2 qubits (ej. CNOT) cuestan `15.0` debido a su complejidad en hardware de microondas o iones atrapados.
2. **Reloj por Qubit**: Mantenemos un reloj para cada qubit.
3. **Sincronización (Ruta Crítica)**: Iteramos el `Graph` topológicamente (por columnas). Cuando una compuerta involucra múltiples qubits, esta *no puede ejecutarse hasta que todos los qubits involucrados estén libres*. El tiempo de inicio será el `máximo` de los relojes de esos qubits.
4. **Camino Crítico**: El tiempo total de una sola ejecución (un "shot") es el valor máximo alcanzado por cualquier reloj al finalizar el circuito.
5. **Tiempo Total**: Multiplicamos este camino crítico por la cantidad de `shots`, sumándole el retraso físico de reseteo de la QPU.

---

## Modelos de Facturación

El módulo abstrae las diferencias en cómo los proveedores cobran por el acceso cuántico:

- **IBM Quantum**: Facturación por **tiempo puro de uso de la QPU**. El módulo convierte el tiempo heurístico abstracto a milisegundos/segundos reales usando un factor de conversión base (`base_time_ns`) y lo multiplica por el costo por segundo del plan (ej. *Pay-As-You-Go*).
- **AWS Braket**: Facturación **plana**. Ignora el tiempo de ejecución de la ruta crítica y cobra una tarifa base por *Tarea* sumada a una tarifa por *Shot*, la cual varía según el hardware de terceros seleccionado (IonQ, Rigetti, OQC, etc).

---

## Estructura del Módulo

- `src/models.rs`: Estructuras de datos, `BackendProfile`, y esquemas para los reportes de costos.
- `src/scraper.rs`: Lógica asíncrona para conectarse a las APIs de precios de AWS e IBM y gestionar la caché.
- `src/time_estimator.rs`: Algoritmo matemático del DAG y Ruta Crítica.
- `src/calculator.rs`: Lógica de cruce de datos financieros para arrojar las cotizaciones finales en dólares.

---

## Ejemplo de Uso (API)

```rust
use qctidy_estimator::{BackendProfile, estimate_execution_time, get_pricing_data, calculate_costs};
use qctidy::Graph;

// 1. Obtener el Grafo (circuito)
// let graph = ...

// 2. Definir perfil de hardware y simulaciones
let profile = BackendProfile::default();
let shots = 1000;

// 3. Simular el tiempo de ejecución físico
let heuristic_time = estimate_execution_time(&graph, shots, &profile);

// 4. Extraer precios de internet asíncronamente (false = usar caché si está disponible)
let pricing_data = get_pricing_data(false).await;

// 5. Obtener cotizaciones financieras en USD
let costs = calculate_costs(&pricing_data, heuristic_time, shots, profile.base_time_ns);

println!("Costos estimados: {:#?}", costs);
```
