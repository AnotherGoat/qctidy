# QCTidy VS Code Extension

Esta es la extensión oficial de Visual Studio Code para **QCTidy**. Su propósito es proporcionar una interfaz visual en el editor para analizar código Python e identificar componentes clave del código, con soporte especializado para circuitos cuánticos de Qiskit.

## Características Principales

La extensión se integra directamente en la barra lateral (Activity Bar) de VS Code y proporciona las siguientes funcionalidades:

* **Análisis AST en Tiempo Real**: Utiliza `web-tree-sitter` para generar un Árbol de Sintaxis Abstracta (AST) del archivo Python actualmente abierto en el editor.
* **Detección de Clases y Métodos**: Identifica automáticamente las declaraciones de clases (`class_definition`) y funciones (`function_definition`) en el código.
* **Detección de Qiskit**: Identifica instancias y llamadas al constructor de circuitos cuánticos (`QuantumCircuit`).
* **Navegación Rápida**: Al hacer clic en cualquier elemento detectado en el panel lateral, el editor navega automáticamente a la línea y columna exacta donde se encuentra dicho elemento.

## Arquitectura Técnica

Para mantener el proyecto ligero y sin dependencias innecesarias, la extensión fue construida desde cero sin utilizar generadores de código estándar (`yo code`). 

Los componentes principales son:

1. **`src/extension.ts`**: Punto de entrada de la extensión. Se encarga de inicializar el entorno, registrar el comando de navegación (`qctidy.jumpToLine`) y suscribir el proveedor de datos de la vista al evento de cambio de editor.
2. **`src/parser.ts`**: Gestiona la inicialización de `web-tree-sitter` y carga las reglas gramaticales para Python (`tree-sitter-python.wasm`) compiladas en WebAssembly. Para solucionar incompatibilidades de tipos estáticos con la versión más reciente de la librería (v0.23.0), se utiliza un enfoque de importación CommonJS estándar (`require`).
3. **`src/treeDataProvider.ts`**: Implementa la interfaz `vscode.TreeDataProvider`. Recorre el AST generado y mapea los nodos relevantes (Clases, Métodos y Circuitos) a elementos visuales (`AstNodeItem`) configurados con íconos nativos del tema de VS Code.

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
