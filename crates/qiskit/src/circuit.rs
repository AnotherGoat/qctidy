use pyo3::prelude::*;
use qsimplify::{Circuit, GateOperation};

pub(crate) fn circuit_to_qiskit(python: Python<'_>, circuit: &Circuit) -> PyResult<Py<PyAny>> {
    use GateOperation::*;

    let qiskit = python.import("qiskit")?;
    let quantum_circuit_class = qiskit.getattr("QuantumCircuit")?;

    let qubit_count = circuit
        .operations()
        .iter()
        .flat_map(GateOperation::qubits)
        .max()
        .map_or(0, |qubit| qubit + 1);

    let bit_count = circuit
        .operations()
        .iter()
        .flat_map(GateOperation::bits)
        .max()
        .map_or(0, |bit| bit + 1);

    let qiskit_circuit = quantum_circuit_class.call1((qubit_count, bit_count))?;

    for operation in circuit.operations() {
        match *operation {
            ID { qubit } => {
                qiskit_circuit.call_method1("id", (qubit,))?;
            }
            H { qubit } => {
                qiskit_circuit.call_method1("h", (qubit,))?;
            }
            X { qubit } => {
                qiskit_circuit.call_method1("x", (qubit,))?;
            }
            Y { qubit } => {
                qiskit_circuit.call_method1("y", (qubit,))?;
            }
            Z { qubit } => {
                qiskit_circuit.call_method1("z", (qubit,))?;
            }
            P { angle, qubit } => {
                qiskit_circuit.call_method1("p", (angle, qubit))?;
            }
            RX { angle, qubit } => {
                qiskit_circuit.call_method1("rx", (angle, qubit))?;
            }
            RY { angle, qubit } => {
                qiskit_circuit.call_method1("ry", (angle, qubit))?;
            }
            RZ { angle, qubit } => {
                qiskit_circuit.call_method1("rz", (angle, qubit))?;
            }
            S { qubit } => {
                qiskit_circuit.call_method1("s", (qubit,))?;
            }
            SDG { qubit } => {
                qiskit_circuit.call_method1("sdg", (qubit,))?;
            }
            SX { qubit } => {
                qiskit_circuit.call_method1("sx", (qubit,))?;
            }
            SY { qubit } => {
                qiskit_circuit.call_method1("sy", (qubit,))?;
            }
            T { qubit } => {
                qiskit_circuit.call_method1("t", (qubit,))?;
            }
            TDG { qubit } => {
                qiskit_circuit.call_method1("tdg", (qubit,))?;
            }
            Measure { qubit, bit } => {
                qiskit_circuit.call_method1("measure", (qubit, bit))?;
            }
            Swap { qubit1, qubit2 } => {
                qiskit_circuit.call_method1("swap", (qubit1, qubit2))?;
            }
            CH { control, target } => {
                qiskit_circuit.call_method1("ch", (control, target))?;
            }
            CX { control, target } => {
                qiskit_circuit.call_method1("cx", (control, target))?;
            }
            CY { control, target } => {
                qiskit_circuit.call_method1("cy", (control, target))?;
            }
            CZ { qubit1, qubit2 } => {
                qiskit_circuit.call_method1("cz", (qubit1, qubit2))?;
            }
            CP {
                angle,
                qubit1,
                qubit2,
            } => {
                qiskit_circuit.call_method1("cp", (angle, qubit1, qubit2))?;
            }
            CSwap {
                control,
                target1,
                target2,
            } => {
                qiskit_circuit.call_method1("cswap", (control, target1, target2))?;
            }
            CCX {
                control1,
                control2,
                target,
            } => {
                qiskit_circuit.call_method1("ccx", (control1, control2, target))?;
            }
            CCZ {
                qubit1,
                qubit2,
                qubit3,
            } => {
                qiskit_circuit.call_method1("ccz", (qubit1, qubit2, qubit3))?;
            }
        }
    }

    Ok(qiskit_circuit.into())
}
