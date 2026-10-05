# QCTidy VS Code Extension

Esta es la extensión oficial de Visual Studio Code para **QCTidy**. Su propósito es proporcionar una interfaz visual en el editor para analizar código Python e identificar componentes clave del código, con soporte especializado para circuitos cuánticos de Qiskit.

## Características Principales

La extensión se integra directamente en la barra lateral (Activity Bar) de VS Code y proporciona las siguientes funcionalidades:

* **Análisis AST en Tiempo Real**: Utiliza `web-tree-sitter` para generar un Árbol de Sintaxis Abstracta (AST) del archivo Python actualmente abierto en el editor.
* **Jerarquía Completa**: Estructura en árbol colapsable y expandible similar al explorador de archivos:
  $$\text{Función/Método/Clase} \longrightarrow \text{Circuito Cuántico} \longrightarrow \text{Metadatos y Compuertas} \longrightarrow \text{Parámetros}$$
* **Detección Multivariable y Nombres de Circuitos**: Identifica todas las variables asignadas a `QuantumCircuit` mostrando el nombre exacto de la variable.
* **Extracción de Metadatos**: Extrae y muestra el número de cúbits y bits clásicos definidos para cada circuito.
* **Detección Secuencial de Compuertas**: Mapea todas las compuertas aplicadas al circuito (1, 2 y 3 cúbits, rotaciones, mediciones).
* **Detección Especial para $\sqrt{Y}$**: Reconoce construcciones como `circuit.append(YGate().power(1 / 2), [0])` como compuerta $\sqrt{Y}$ (`SY`), identificando su potencia y cúbit.
* **Desglose de Parámetros**: Permite desplegar cada compuerta para inspeccionar sus parámetros individuales (ángulos, cúbits de control y objetivo).
* **Navegación Rápida**: Al hacer clic en cualquier función, circuito o compuerta en el panel lateral, el editor salta automáticamente a su línea y columna precisa.

## Arquitectura Técnica

Para mantener el proyecto ligero y sin dependencias innecesarias, la extensión fue construida desde cero sin utilizar generadores de código estándar (`yo code`). 

Los componentes principales son:

1. **`src/extension.ts`**: Punto de entrada de la extensión. Se encarga de inicializar el entorno, registrar el comando de navegación (`qctidy.jumpToLine`) y suscribir el proveedor de datos de la vista al evento de cambio de editor.
2. **`src/parser.ts`**: Gestiona la inicialización de `web-tree-sitter` y carga las reglas gramaticales para Python (`tree-sitter-python.wasm`) compiladas en WebAssembly.
3. **`src/treeDataProvider.ts`**: Implementa la interfaz `vscode.TreeDataProvider`. Analiza recursivamente el AST generado, construyendo la jerarquía de Clases/Funciones, Circuitos, Metadatos, Compuertas y Parámetros con sus respectivos íconos nativos y comandos de navegación.

## Entorno de Desarrollo (Pruebas Locales)

Para probar, depurar o extender esta extensión localmente:

1. Asegúrate de tener Node.js instalado.
2. Abre la carpeta `vscode/` en Visual Studio Code.
3. Instala las dependencias ejecutando:
   ```bash
   npm install
   ```
4. Presiona `F5` en tu teclado. Esto ejecutará automáticamente la tarea de compilación de TypeScript (ver `.vscode/tasks.json`) y abrirá una nueva ventana del editor ("Extension Development Host") con la extensión cargada.
5. Abre cualquier archivo `.py` en la nueva ventana para ver la extensión en acción.
