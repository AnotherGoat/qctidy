# QCTidy VS Code Extension

Esta es la extensión oficial de Visual Studio Code para **QCTidy**. Su propósito es proporcionar una interfaz visual en el editor para analizar código Python e identificar componentes clave del código, con soporte especializado para circuitos cuánticos de Qiskit.

## Características Principales

La extensión se integra directamente en la barra lateral (Activity Bar) de VS Code y proporciona las siguientes funcionalidades:

* **Análisis AST en Tiempo Real**: Invoca `qctidy check --output-format json`, cuyo informe incluye el análisis del archivo Python actualmente abierto en el editor.
* **Jerarquía Completa**: Estructura en árbol colapsable y expandible similar al explorador de archivos:
  $$\text{Función/Método/Clase} \longrightarrow \text{Circuito Cuántico} \longrightarrow \text{Metadatos y Compuertas} \longrightarrow \text{Parámetros}$$
* **Detección Multivariable y Nombres de Circuitos**: Identifica todas las variables asignadas a `QuantumCircuit` mostrando el nombre exacto de la variable. Si varios circuitos comparten el mismo nombre de variable, se desambiguan como `nombre:línea`.
* **Extracción de Metadatos**: Extrae y muestra el número de cúbits y bits clásicos definidos para cada circuito.
* **Detección Secuencial de Compuertas**: Mapea todas las compuertas aplicadas al circuito (1, 2 y 3 cúbits, rotaciones, mediciones).
* **Detección Especial para $\sqrt{Y}$**: Reconoce construcciones como `circuit.append(YGate().power(1 / 2), [0])` como compuerta $\sqrt{Y}$ (`SY`), identificando su potencia y cúbit.
* **Desglose de Parámetros**: Permite desplegar cada compuerta para inspeccionar sus parámetros individuales (ángulos, cúbits de control y objetivo).
* **Navegación Rápida**: Al hacer clic en cualquier función, circuito, compuerta o parámetro, el editor salta a su línea y columna exacta y, si el nodo es expandible, también se expande automáticamente. Los parámetros de una llamada multilínea saltan a su propia línea.
* **Versión de Qiskit**: Muestra la versión de Qiskit detectada en el entorno, o `Qiskit: not found` si no se encuentra.
* **Mensajes de Estado**: Si el archivo abierto no es `.py`, o si no contiene circuitos, el panel lo indica explícitamente.
* **Recarga Manual**: Botón en la esquina superior derecha del panel para volver a analizar el archivo y redetectar la versión de Qiskit.

## Arquitectura Técnica

Para mantener el proyecto ligero y sin dependencias innecesarias, la extensión fue construida desde cero sin utilizar generadores de código estándar (`yo code`).

Los componentes principales son:

1. **`src/extension.ts`**: Punto de entrada de la extensión. Inicializa el entorno, crea la vista de árbol, supervisa la disponibilidad de QCTidy y registra los comandos de navegación y recarga (`qctidy.openNode`, `qctidy.refresh`).
2. **`src/cli.ts`**: Cliente unificado del CLI de QCTidy. Gestiona la resolución del ejecutable (`qctidy.path` o `PATH`, con soporte multiplataforma y expansión de `~`), verificación de disponibilidad (`qctidy --version`) con timeout y caché, ejecución desacoplada por `spawn` (sin shell y con soporte de `stdin`), y ejecución de `check`.
3. **`src/ast.ts`**: Tipos y modelos de datos del análisis Python incluido en el informe de `qctidy check`.
4. **`src/treeDataProvider.ts`**: Implementa la interfaz `vscode.TreeDataProvider`. Recorre el análisis devuelto por el CLI y construye la jerarquía de Clases/Funciones, Circuitos, Metadatos, Compuertas y Parámetros con sus respectivos íconos nativos y comandos de navegación. También reporta el estado y versión tanto de QCTidy como de Qiskit.
5. **`src/qiskit.ts`**: Detecta la versión de Qiskit usando el intérprete configurado en la extensión de Python (con el `PATH` como respaldo) y se vuelve a detectar cuando cambia el intérprete.

## Entorno de Desarrollo (Pruebas Locales)

Para probar, depurar o extender esta extensión localmente:

1. Asegúrate de tener Node.js instalado y el binario `qctidy` disponible en el `PATH` (por ejemplo, tras `cargo build --release -p qctidy-cli`).
2. Instala las dependencias de la extensión, dentro de `vscode/`:
   ```bash
   cd vscode
   npm install
   ```
3. Abre la **raíz del repositorio** en Visual Studio Code (no la carpeta `vscode/`).
4. Presiona `F5`. La configuración de depuración de la raíz (`.vscode/launch.json`) se encarga de:
   - ejecutar la tarea `qctidy: compile extension` (`.vscode/tasks.json`), que corre `npm run compile` dentro de `vscode/`;
   - lanzar una ventana "Extension Development Host" con la extensión cargada;
   - abrir la carpeta `vscode/test` como workspace y, dentro de ella, el archivo de prueba `vscode/test/test_file.py`.
5. En la ventana de desarrollo, el panel **QCTidy** de la barra lateral muestra el AST del archivo abierto. Puedes abrir cualquier otro archivo `.py` para analizarlo.

### Recarga manual

En la esquina superior derecha del panel hay un botón de recarga (ícono `refresh`) que vuelve a analizar el archivo y a detectar la versión de Qiskit, útil cuando el modo automático no se actualiza.
