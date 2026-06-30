# QSimplify Frontend 

Esta es la interfaz gráfica oficial para **QSimplify**, un simplificador de circuitos cuánticos en Rust. La interfaz permite a los usuarios diseñar circuitos arrastrando y soltando compuertas cuánticas, y comunicarse con la API en Rust para obtener la versión matemáticamente simplificada del circuito.

## Características (Features)

* **Diseño Interfaz Gráfica**: Interfaz moderna basada en "Glassmorphism" con un tema oscuro elegante y animaciones fluidas, utilizando TailwindCSS.
* **Drag & Drop (Arrastrar y Soltar)**: Sistema robusto construido sobre `@dnd-kit/core`. Permite mover compuertas libremente, insertarlas en medio del circuito, y eliminarlas arrastrándolas fuera de la cuadrícula.
* **Edición Interactiva**: Al hacer clic en las compuertas, se abre un menú flotante para configurar sus parámetros matemáticos en tiempo real (por ejemplo, definir los grados para las compuertas de rotación como `Rx` o asignar el cable clásico para la medición `M`).
* **Integración con API Backend**: Comunicación automática y sin problemas de CORS con el servidor de Rust (DuckDNS) a través del proxy de Vite.
* **Compuertas Soportadas**:
  * *Básicas*: I, H, X, Y, Z, S, Sdg.
  * *Rotaciones*: P, Rx, Ry, Rz, Sx, Sy, T, Tdg.
  * *Operaciones*: M (Medición).
* **Responsive & Adaptable**: Permite ampliar el número de qubits (filas) de manera dinámica.

## Tecnologías Utilizadas

* **Framework**: [React Router](https://reactrouter.com/) (v7)
* **Build Tool**: [Vite](https://vitejs.dev/)
* **Estilos**: [Tailwind CSS](https://tailwindcss.com/)
* **Drag & Drop**: [@dnd-kit](https://dndkit.com/)
* **Iconos**: [Lucide React](https://lucide.dev/)

---

## Guía de Inicio (Getting Started)

### 1. Instalación de Dependencias

Asegúrate de tener [Node.js](https://nodejs.org/) instalado. Luego, ejecuta:

```bash
npm install
```

### 2. Entorno de Desarrollo Local

Para iniciar el servidor de desarrollo con Hot Module Replacement (HMR):

```bash
npm run dev
```

Tu aplicación estará disponible en `http://localhost:5173`.

> **Nota sobre la API**: El proyecto está configurado para utilizar un Proxy en Vite (`vite.config.ts`) que redirige todas las llamadas de `/api` hacia `https://qsimplify.duckdns.org` de manera invisible para evitar problemas de CORS en el navegador.

---

## Estructura del Proyecto

* `app/components/`
  * `circuit.tsx`: Componente principal que dibuja las filas de los qubits y maneja la renderización de las celdas.
  * `gate.tsx`: Definición visual y lógica de las compuertas cuánticas (colores, tamaños, IDs).
  * `gate-sidebar.tsx`: Panel lateral con el catálogo completo de compuertas disponibles.
  * `gate-editor.tsx`: Menú flotante interactivo para configurar los ángulos y bits de las compuertas.
* `app/lib/`
  * `api.ts`: Lógica de conexión (Fetch) para enviar los circuitos a la API de Rust y recibir la respuesta.
* `app/routes/`
  * `home.tsx`: La vista principal. Controla los estados de React, gestiona los eventos globales de Drag & Drop y la llamada al simplificador.

---

## Construcción para Producción

Para compilar la aplicación para su despliegue a producción:

```bash
npm run build
```

Esto generará la carpeta `build/` con el código optimizado, listo para ser alojado en servicios como Vercel, Netlify, o servido directamente desde el backend en Rust.