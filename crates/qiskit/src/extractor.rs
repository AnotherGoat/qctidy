use std::str::FromStr;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::{PyAny, PyResult};

use qsimplify::domain::GateType;
use qsimplify::dto::gate::GateOperation;

pub(crate) fn extract_operations(circuit: &Bound<'_, PyAny>) -> PyResult<Vec<GateOperation>> {
    let data = circuit.getattr("data")?;
    let mut operations = Vec::new();

    for item in data.try_iter()? {
        let item = item?;

        if let Some(operation) = parse_instruction(circuit, &item)? {
            operations.push(operation);
        }
    }

    Ok(operations)
}

fn parse_instruction(
    circuit: &Bound<'_, PyAny>,
    instruction: &Bound<'_, PyAny>,
) -> PyResult<Option<GateOperation>> {
    let operation = instruction.getattr("operation")?;
    let name: String = operation.getattr("name")?.extract()?;

    if name == "barrier" {
        return Ok(None);
    }

    let r#type = extract_gate_type(&name)?;
    let qubits = extract_qubit_indices(circuit, instruction)?;
    let bits = extract_bit_indices(circuit, instruction)?;
    let parameters = extract_parameters(&operation)?;

    Ok(Some(GateOperation {
        r#type,
        qubits,
        bits,
        parameters,
    }))
}

fn extract_gate_type(name: &str) -> PyResult<GateType> {
    match GateType::from_str(name) {
        Ok(r#type) => Ok(r#type),
        Err(_) => Err(PyValueError::new_err(format!("Unsupported gate: {}", name))),
    }
}

fn extract_qubit_indices(
    circuit: &Bound<'_, PyAny>,
    instruction: &Bound<'_, PyAny>,
) -> PyResult<Vec<usize>> {
    let python_qubits = instruction.getattr("qubits")?;
    let mut qubits = Vec::new();

    for qubit in python_qubits.try_iter()? {
        let qubit = qubit?;

        let bit_locations = circuit.getattr("find_bit")?.call1((qubit,))?;
        let registers = bit_locations.getattr("registers")?;

        for entry in registers.try_iter()? {
            let entry = entry?;

            let qubit_index: usize = entry.get_item(1)?.extract()?;
            qubits.push(qubit_index);
        }
    }

    Ok(qubits)
}

fn extract_bit_indices(
    circuit: &Bound<'_, PyAny>,
    instruction: &Bound<'_, PyAny>,
) -> PyResult<Vec<usize>> {
    let python_bits = instruction.getattr("clbits")?;
    let mut bits = Vec::new();

    for bit in python_bits.try_iter()? {
        let bit = bit?;

        let bit_locations = circuit.getattr("find_bit")?.call1((bit,))?;
        let registers = bit_locations.getattr("registers")?;

        for entry in registers.try_iter()? {
            let entry = entry?;

            let bit_index: usize = entry.get_item(1)?.extract()?;
            bits.push(bit_index);
        }
    }

    Ok(bits)
}

fn extract_parameters(operation: &Bound<'_, PyAny>) -> PyResult<Vec<f64>> {
    let python_parameters = operation.getattr("params")?;
    let parameters: Vec<f64> = python_parameters.extract()?;
    Ok(parameters)
}
