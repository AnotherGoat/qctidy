use pyo3::prelude::*;
use qsimplify::GateOperation;

pub(crate) fn operations_to_circuit(
    python: Python<'_>,
    operations: &[GateOperation],
) -> PyResult<Py<PyAny>> {
    use GateOperation::*;

    let qiskit = python.import("qiskit")?;
    let quantum_circuit_class = qiskit.getattr("QuantumCircuit")?;

    let qubit_count = operations
        .iter()
        .flat_map(GateOperation::qubits)
        .max()
        .map_or(0, |qubit| qubit + 1);

    let bit_count = operations
        .iter()
        .flat_map(GateOperation::bits)
        .max()
        .map_or(0, |bit| bit + 1);

    let circuit = quantum_circuit_class.call1((qubit_count, bit_count))?;

    for operation in operations {
        match *operation {
            ID { .. } => {}
            H { qubit } => {
                circuit.call_method1("h", (qubit,))?;
            }
            X { qubit } => {
                circuit.call_method1("x", (qubit,))?;
            }
            Y { qubit } => {
                circuit.call_method1("y", (qubit,))?;
            }
            Z { qubit } => {
                circuit.call_method1("z", (qubit,))?;
            }
            P { angle, qubit } => {
                circuit.call_method1("p", (angle, qubit))?;
            }
            RX { angle, qubit } => {
                circuit.call_method1("rx", (angle, qubit))?;
            }
            RY { angle, qubit } => {
                circuit.call_method1("ry", (angle, qubit))?;
            }
            RZ { angle, qubit } => {
                circuit.call_method1("rz", (angle, qubit))?;
            }
            S { qubit } => {
                circuit.call_method1("s", (qubit,))?;
            }
            SDG { qubit } => {
                circuit.call_method1("sdg", (qubit,))?;
            }
            SX { qubit } => {
                circuit.call_method1("sx", (qubit,))?;
            }
            SY { qubit } => {
                circuit.call_method1("sy", (qubit,))?;
            }
            T { qubit } => {
                circuit.call_method1("t", (qubit,))?;
            }
            TDG { qubit } => {
                circuit.call_method1("tdg", (qubit,))?;
            }
            Measure { qubit, bit } => {
                circuit.call_method1("measure", (qubit, bit))?;
            }
            Swap { qubit1, qubit2 } => {
                circuit.call_method1("swap", (qubit1, qubit2))?;
            }
            CH { control, target } => {
                circuit.call_method1("ch", (control, target))?;
            }
            CX { control, target } => {
                circuit.call_method1("cx", (control, target))?;
            }
            CY { control, target } => {
                circuit.call_method1("cy", (control, target))?;
            }
            CZ { qubit1, qubit2 } => {
                circuit.call_method1("cz", (qubit1, qubit2))?;
            }
            CP {
                angle,
                qubit1,
                qubit2,
            } => {
                circuit.call_method1("cp", (angle, qubit1, qubit2))?;
            }
            CSwap {
                control,
                target1,
                target2,
            } => {
                circuit.call_method1("cswap", (control, target1, target2))?;
            }
            CCX {
                control1,
                control2,
                target,
            } => {
                circuit.call_method1("ccx", (control1, control2, target))?;
            }
            CCZ {
                qubit1,
                qubit2,
                qubit3,
            } => {
                circuit.call_method1("ccz", (qubit1, qubit2, qubit3))?;
            }
        }
    }

    Ok(circuit.into())
}
