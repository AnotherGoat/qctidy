use std::str::FromStr;
use std::sync::LazyLock;

use faer::Mat;
use faer::complex::Complex64;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use qsimplify::GateType;
use qsimplify::dto::GateOperation;

#[expect(clippy::unnested_or_patterns)]
static EXPECTED_SY: LazyLock<Mat<Complex64>> = LazyLock::new(|| {
    Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) | (1, 0) | (1, 1) => Complex64::new(0.5_f64, 0.5_f64),
        (0, 1) => Complex64::new(-0.5_f64, -0.5_f64),
        _ => Complex64::new(0.0_f64, 0.0_f64),
    })
});

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
    use GateType::*;

    let operation = instruction.getattr("operation")?;
    let name: String = operation.getattr("name")?.extract()?;

    if name == "barrier" {
        return Ok(None);
    }

    if name == "unitary" {
        if is_unitary_sy(&operation)? {
            let qubits = extract_qubit_indices(circuit, instruction)?;
            let qubit = qubits
                .first()
                .ok_or_else(|| PyValueError::new_err("SY gate is missing a qubit"))?;

            return Ok(Some(GateOperation::sy(*qubit)));
        }
        return Err(PyValueError::new_err(
            "Unsupported unitary gate (only SY is allowed)",
        ));
    }

    let r#type = extract_gate_type(&name)?;
    let qubits = extract_qubit_indices(circuit, instruction)?;
    let bits = extract_bit_indices(circuit, instruction)?;
    let parameters = extract_parameters(&operation)?;

    let qubit0 = get_qubit(&qubits, 0)?;

    let operation = match r#type {
        ID => return Ok(None),
        H => GateOperation::h(qubit0),
        X => GateOperation::x(qubit0),
        Y => GateOperation::y(qubit0),
        Z => GateOperation::z(qubit0),
        S => GateOperation::s(qubit0),
        SDG => GateOperation::sdg(qubit0),
        SX => GateOperation::sx(qubit0),
        SY => GateOperation::sy(qubit0),
        T => GateOperation::t(qubit0),
        TDG => GateOperation::tdg(qubit0),
        P => GateOperation::p(get_parameter(&parameters, 0)?, qubit0),
        RX => GateOperation::rx(get_parameter(&parameters, 0)?, qubit0),
        RY => GateOperation::ry(get_parameter(&parameters, 0)?, qubit0),
        RZ => GateOperation::rz(get_parameter(&parameters, 0)?, qubit0),
        Measure => GateOperation::measure(qubit0, get_bit(&bits, 0)?),
        Swap => GateOperation::swap(qubit0, get_qubit(&qubits, 1)?),
        CH => GateOperation::ch(qubit0, get_qubit(&qubits, 1)?),
        CX => GateOperation::cx(qubit0, get_qubit(&qubits, 1)?),
        CY => GateOperation::cy(qubit0, get_qubit(&qubits, 1)?),
        CZ => GateOperation::cz(qubit0, get_qubit(&qubits, 1)?),
        CP => GateOperation::cp(
            get_parameter(&parameters, 0)?,
            qubit0,
            get_qubit(&qubits, 1)?,
        ),
        CSwap => GateOperation::c_swap(qubit0, get_qubit(&qubits, 1)?, get_qubit(&qubits, 2)?),
        CCX => GateOperation::ccx(qubit0, get_qubit(&qubits, 1)?, get_qubit(&qubits, 2)?),
        CCZ => GateOperation::ccz(qubit0, get_qubit(&qubits, 1)?, get_qubit(&qubits, 2)?),
    };

    Ok(Some(operation))
}

fn extract_gate_type(name: &str) -> PyResult<GateType> {
    GateType::from_str(name).map_or_else(
        |_| Err(PyValueError::new_err(format!("Unsupported gate: {name}"))),
        Ok,
    )
}

fn is_unitary_sy(operation: &Bound<'_, PyAny>) -> PyResult<bool> {
    let name: String = operation.getattr("name")?.extract()?;

    if name != "unitary" {
        return Ok(false);
    }

    let qubit_count: usize = operation.getattr("num_qubits")?.extract()?;
    let bit_count: usize = operation.getattr("num_clbits")?.extract()?;

    if qubit_count != 1 || bit_count != 0 {
        return Ok(false);
    }

    let parameters = operation.getattr("params")?;
    let parameters: Vec<Bound<'_, PyAny>> = parameters.extract()?;

    if parameters.len() != 1 {
        return Ok(false);
    }

    let matrix = &parameters
        .first()
        .expect("There should be exactly one parameter");
    let extracted: Vec<Vec<(f64, f64)>> = matrix.extract()?;

    if extracted.len() != 2 || extracted[0].len() != 2 {
        return Ok(false);
    }

    for row in 0..2 {
        for column in 0..2 {
            let (real, imaginary) = extracted[row][column];
            let Complex64 {
                re: expected_real,
                im: expected_imaginary,
            } = EXPECTED_SY[(row, column)];

            if (real - expected_real).abs() > qsimplify::EPSILON
                || (imaginary - expected_imaginary).abs() > qsimplify::EPSILON
            {
                return Ok(false);
            }
        }
    }

    Ok(true)
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

            let qubit_index: usize = entry.get_item(1_i32)?.extract()?;
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

            let bit_index: usize = entry.get_item(1_i32)?.extract()?;
            bits.push(bit_index);
        }
    }

    Ok(bits)
}

fn extract_parameters(operation: &Bound<'_, PyAny>) -> PyResult<Vec<f64>> {
    let python_parameters = operation.getattr("params")?;

    let parameters: Vec<f64> = python_parameters
        .extract()
        .map_err(|_| PyValueError::new_err("Failed to extract gate parameters as floats"))?;

    Ok(parameters)
}

fn get_qubit(qubits: &[usize], index: usize) -> Result<usize, PyErr> {
    qubits
        .get(index)
        .copied()
        .ok_or_else(|| PyValueError::new_err(format!("Missing qubit at index {index}")))
}

fn get_bit(bits: &[usize], index: usize) -> Result<usize, PyErr> {
    bits.get(index)
        .copied()
        .ok_or_else(|| PyValueError::new_err(format!("Missing bit at index {index}")))
}

fn get_parameter(parameters: &[f64], index: usize) -> Result<f64, PyErr> {
    parameters
        .get(index)
        .copied()
        .ok_or_else(|| PyValueError::new_err(format!("Missing parameter at index {index}")))
}
