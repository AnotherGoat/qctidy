# QSimplify Analyzer (`qsimplify-analyzer`)

Este *crate* está diseñado para abstraer y realizar todo el análisis métrico de los circuitos cuánticos generados por `qsimplify`. Su objetivo principal es extraer estadísticas vitales del `Graph` que luego pueden ser consumidas por:
- **Interfaces Gráficas (Web/TUI):** Para generar reportes de "Antes vs. Después" de las simplificaciones.
- **Motores de Costos (Estimator):** Para calcular el precio de ejecución en hardware cuántico (AWS Braket, IBM Quantum) basándose en las compuertas utilizadas y la profundidad.

## Arquitectura y Componentes

Este analizador está diseñado como un paquete independiente para mantener la separación de responsabilidades (*Separation of Concerns*) y no sobrecargar la librería core. Cuenta con dos sub-módulos principales:

### 1. `metrics.rs` (Estructuras de Datos)
Define las estructuras fuertemente tipadas que representan los reportes. Todas implementan los *traits* de derivación estándar (`Debug`, `Clone`, `Copy`, `PartialEq`).
- **`Metrics`**: Contiene el conteo básico de un circuito (cantidad de qubits, profundidad, e instancias específicas de compuertas como X, Y, Z, Hadamard, etc.).
- **`DeltaMetrics`**: Estructura opcional (`Option<isize>`) que calcula la diferencia (resta matemática) entre dos métricas. Ideal para evaluar la "mejora" de un circuito tras ser pasado por el optimizador.
- **`DetailedMetrics`**: Ofrece un análisis profundo, calculando:
  - Densidad máxima y densidad por columna.
  - Porcentajes de uso de compuertas rotacionales y controladas.
  - Concentración de compuertas complejas (Toffoli / Ancilla).

#### Formato Legible para Humanos (`Display`)
Se implementó el trait `std::fmt::Display` para `DetailedMetrics`, lo cual permite que cualquier interfaz de consola (TUI) o CLI pueda imprimir un reporte estético y profesional del circuito sin necesidad de programar formateos adicionales.

#### Serialización JSON
> **Nota de Serialización:** Todas las estructuras de métricas implementan `serde::{Serialize, Deserialize}`. Esto garantiza que el analizador está 100% listo para ser integrado en el servidor web (`crates/server`), serializando las métricas a JSON de forma instantánea para el Frontend.

### 2. `calculator.rs` (Motor Matemático)
Contiene las funciones públicas (`calculate_metrics`, `calculate_detailed_metrics`, `compare_metrics`) que interactúan directamente con el Grafo.
- **Eficiencia y Exactitud:** El cálculo evita crear arreglos temporales pesados en memoria. En su lugar, itera directamente sobre `graph.iter_nodes()`. Agrupa inteligentemente las compuertas para evitar el "Doble Conteo" (por ejemplo, una compuerta CNOT que ocupa 2 qubits es contabilizada como 1 sola compuerta lógica dividiendo por su peso).
- **Seguridad (Zero-Division Protection):** Todos los cálculos de porcentajes y densidades están protegidos mediante bloques lógicos. Si se analiza un circuito totalmente vacío, el analizador detecta la posible división por cero y devuelve promedios de `0.0`, asegurando que el programa jamás colapse (previniendo *panics* en tiempo de ejecución).

## Pruebas Unitarias (Testing)

El motor de cálculo cuenta con una batería de pruebas unitarias integradas en `calculator.rs` para garantizar la precisión matemática bajo cualquier condición:

- **`test_calculate_metrics`:** Construye programáticamente un circuito cuántico utilizando el `GraphBuilder` inyectando compuertas Hadamard y CNOT. Luego verifica aserciones estrictas sobre el conteo resultante (asegurando que `metrics.h_count == 1`, `metrics.cx_count == 1`, etc.).
- **`test_empty_graph_does_not_panic`:** Prueba de estrés (Edge case) donde se le inyecta un grafo instanciado pero sin compuertas. Verifica que los mecanismos de protección contra división por cero se activen correctamente y el analizador retorne valores seguros sin colapsar.

## Dependencias externas
- `serde` (con feature `derive`): Habilita la interoperabilidad con servicios web.
