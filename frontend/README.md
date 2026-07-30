# QSimplify Frontend

This is the official graphical interface for **QSimplify**, a quantum circuit simplifier written in Rust. The interface lets users design circuits by dragging and dropping quantum gates, then communicate with the Rust API to get the mathematically simplified version of the circuit.

## Features

- **Graphical Interface Design**: Modern glassmorphism-based interface with an elegant dark theme and smooth animations, built with Tailwind CSS.
- **Drag & Drop**: Robust system built on `@dnd-kit/core`. It lets users move gates freely, insert them in the middle of the circuit, and remove them by dragging them outside the grid. It now includes **full support for dragging complete multi-qubit gates (2 and 3 qubits)** without losing their original structure or spacing.
- **Interactive Editing**: Clicking a gate opens a floating menu to configure its mathematical parameters in real time, such as setting degrees for rotation gates like `Rx` or assigning the classical wire for measurement `M`. Inputs include strict range validation, and angles are visually formatted with the degree symbol (`°`).
- **Backend API Integration**: Automatic CORS-free communication with the Rust server (DuckDNS) through the Vite proxy. It includes strict bidirectional mapping to ensure gate synthesis (Backend -> Frontend) is graphically reconstructed with full fidelity.
- **Supported Gates**:
  - _Basic_: I, H, X, Y, Z, S, S† (Sdg).
  - _Rotations & Phase_: P, Rx, Ry, Rz, √X (Sx), √Y (Sy), T, T† (Tdg), U (3-angle unitary).
  - _2 Qubits_: SWAP, CX, CY, CZ, CH, CP (including dynamic vertical connection line rendering and strict support for backend schemas using `control/target` or `qubit1/qubit2`).
  - _3 Qubits_: CCX (Toffoli), CCZ, CSWAP (Fredkin) (full visual representation across multiple rows with distinct controls and targets).
  - _Operations_: M (Measurement).
- **Responsive & Adaptable**: Allows the number of qubits (rows) to be expanded dynamically.

## Technologies Used

- **Framework**: [React Router](https://reactrouter.com/) (v7)
- **Build Tool**: [Vite](https://vitejs.dev/)
- **Styles**: [Tailwind CSS](https://tailwindcss.com/)
- **Drag & Drop**: [@dnd-kit](https://dndkit.com/)
- **Icons**: [Lucide React](https://lucide.dev/)

---

## Getting Started

### 1. Install Dependencies

Make sure [Node.js](https://nodejs.org/) is installed. Then run:

```bash
npm install
```

### 2. Local Development Environment

To start the development server with Hot Module Replacement (HMR):

```bash
npm run dev
```

Your application will be available at `http://localhost:5173`.

> **API Note**: The project is configured to use a Vite proxy (`vite.config.ts`) that transparently redirects all `/api` calls to `VITE_API_URL` to avoid browser CORS issues. Configure it in `.env`.

---

## Project Structure

- `app/components/`
  - `circuit.tsx`: Main component that draws qubit rows and handles cell rendering.
  - `gate.tsx`: Visual and logical definition of quantum gates (colors, sizes, IDs).
  - `gate-sidebar.tsx`: Sidebar with the full catalog of available gates.
  - `gate-editor.tsx`: Interactive floating menu for configuring gate angles and bits.
- `app/lib/`
  - `api.ts`: Connection logic (Fetch) for sending circuits to the Rust API and receiving the response.
- `app/routes/`
  - `home.tsx`: The main view. It controls React state, handles global drag-and-drop events, and calls the simplifier.

---

## Production Build

To compile the application for production deployment:

```bash
npm run build
```

This generates the `build/` folder with optimized code, ready to be hosted on services like Vercel or Netlify, or served directly from the Rust backend.
