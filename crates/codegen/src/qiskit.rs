use std::fmt::Write;

use qsimplify::{
    AngleFormat, Circuit, GateOperation, GateType, Graph, PiFormat, formatter::format_angle,
};
use qsimplify_ports::CodeGenerationError;

const PI_FORMAT: PiFormat = PiFormat::Custom { pi: "numpy.pi" };

/// Convert the provided graph into Python code that uses the Qiskit library.
pub(crate) fn generate(graph: &Graph, circuit_name: &str) -> Result<String, CodeGenerationError> {
    let circuit = Circuit::from(graph);
    let mut imports = Vec::<String>::new();
    let mut build_steps = Vec::new();

    add_import(&mut imports, "from qiskit import QuantumCircuit");

    if graph.is_empty() {
        build_steps.push(format!("{circuit_name} = QuantumCircuit()"));
    } else if graph.bits() == 0 {
        build_steps.push(format!(
            "{circuit_name} = QuantumCircuit({})",
            graph.height()
        ));
    } else {
        build_steps.push(format!(
            "{circuit_name} = QuantumCircuit({}, {})",
            graph.height(),
            graph.bits()
        ));
    }

    for operation in circuit.operations() {
        generate_gate(circuit_name, operation, &mut imports, &mut build_steps);
    }

    imports.sort_by(|first, second| {
        let first_parts: Vec<&str> = first.split(' ').collect();
        let second_parts: Vec<&str> = second.split(' ').collect();
        first_parts[1..].cmp(&second_parts[1..])
    });

    let mut output = String::new();

    for import in &imports {
        writeln!(output, "{import}").expect("String should always be writable");
    }

    output.push('\n');

    for step in &build_steps {
        writeln!(output, "{step}").expect("String should always be writable");
    }

    Ok(output.trim_end().to_owned())
}

fn add_import(imports: &mut Vec<String>, import: &str) {
    if !imports.contains(&import.to_owned()) {
        imports.push(import.to_owned());
    }
}

fn generate_gate(
    circuit_name: &str,
    operation: &GateOperation,
    imports: &mut Vec<String>,
    build_steps: &mut Vec<String>,
) {
    use GateOperation::*;

    match *operation {
        ID { qubit }
        | H { qubit }
        | X { qubit }
        | Y { qubit }
        | Z { qubit }
        | S { qubit }
        | SDG { qubit }
        | SX { qubit }
        | T { qubit }
        | TDG { qubit } => {
            add_build_step(circuit_name, qubit, operation, build_steps);
        }
        P { angle, qubit } | RX { angle, qubit } | RY { angle, qubit } | RZ { angle, qubit } => {
            generate_rotation_gate(circuit_name, angle, qubit, operation, imports, build_steps);
        }
        SY { qubit } => {
            imports.push("from qiskit.circuit.library.standard_gates import YGate".to_owned());

            build_steps.push(format!(
                "{circuit_name}.append(YGate().power(1 / 2), [{qubit}])"
            ));
        }
        Measure { qubit, bit } => {
            add_build_step_with_parameters(circuit_name, &[qubit, bit], operation, build_steps);
        }
        Swap { qubit1, qubit2 } | CZ { qubit1, qubit2 } => {
            add_build_step_with_parameters(circuit_name, &[qubit1, qubit2], operation, build_steps);
        }
        CH { control, target } | CX { control, target } | CY { control, target } => {
            add_build_step_with_parameters(
                circuit_name,
                &[control, target],
                operation,
                build_steps,
            );
        }
        CP {
            angle,
            qubit1,
            qubit2,
        } => {
            generate_cp_gate(
                circuit_name,
                angle,
                qubit1,
                qubit2,
                operation,
                imports,
                build_steps,
            );
        }
        CSwap {
            control,
            target1,
            target2,
        } => {
            add_build_step_with_parameters(
                circuit_name,
                &[control, target1, target2],
                operation,
                build_steps,
            );
        }
        CCX {
            control1,
            control2,
            target,
        } => {
            add_build_step_with_parameters(
                circuit_name,
                &[control1, control2, target],
                operation,
                build_steps,
            );
        }
        CCZ {
            qubit1,
            qubit2,
            qubit3,
        } => {
            add_build_step_with_parameters(
                circuit_name,
                &[qubit1, qubit2, qubit3],
                operation,
                build_steps,
            );
        }
    }
}

fn generate_rotation_gate(
    circuit_name: &str,
    angle: f64,
    qubit: usize,
    operation: &GateOperation,
    imports: &mut Vec<String>,
    build_steps: &mut Vec<String>,
) {
    let formatted = format_angle(angle, AngleFormat::Code, PI_FORMAT);

    if formatted == "0" {
        add_build_step_with_string_parameters(
            circuit_name,
            &[&formatted, &qubit.to_string()],
            operation,
            build_steps,
        );
        return;
    }

    if formatted.contains("numpy.pi") {
        add_import(imports, "import numpy");

        add_build_step_with_string_parameters(
            circuit_name,
            &[&formatted, &qubit.to_string()],
            operation,
            build_steps,
        );
        return;
    }

    add_build_step_with_string_parameters(
        circuit_name,
        &[&angle.to_string(), &qubit.to_string()],
        operation,
        build_steps,
    );
}

fn generate_cp_gate(
    circuit_name: &str,
    angle: f64,
    qubit1: usize,
    qubit2: usize,
    operation: &GateOperation,
    imports: &mut Vec<String>,
    build_steps: &mut Vec<String>,
) {
    let formatted = format_angle(angle, AngleFormat::Code, PI_FORMAT);

    if formatted == "0" {
        add_build_step_with_string_parameters(
            circuit_name,
            &[&formatted, &qubit1.to_string(), &qubit2.to_string()],
            operation,
            build_steps,
        );
        return;
    }

    if formatted.contains("numpy.pi") {
        add_import(imports, "import numpy");

        add_build_step_with_string_parameters(
            circuit_name,
            &[&formatted, &qubit1.to_string(), &qubit2.to_string()],
            operation,
            build_steps,
        );
        return;
    }

    add_build_step_with_string_parameters(
        circuit_name,
        &[&angle.to_string(), &qubit1.to_string(), &qubit2.to_string()],
        operation,
        build_steps,
    );
}

fn add_build_step(
    circuit_name: &str,
    qubit: usize,
    operation: &GateOperation,
    build_steps: &mut Vec<String>,
) {
    let gate_name = get_qiskit_gate_name(operation.r#type());

    build_steps.push(format!("{circuit_name}.{gate_name}({qubit})"));
}

fn add_build_step_with_parameters(
    circuit_name: &str,
    parameters: &[usize],
    operation: &GateOperation,
    build_steps: &mut Vec<String>,
) {
    let gate_name = get_qiskit_gate_name(operation.r#type());
    let joined_parameters = parameters
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");

    build_steps.push(format!("{circuit_name}.{gate_name}({joined_parameters})"));
}

fn add_build_step_with_string_parameters(
    circuit_name: &str,
    parameters: &[&str],
    operation: &GateOperation,
    build_steps: &mut Vec<String>,
) {
    let gate_name = get_qiskit_gate_name(operation.r#type());
    let joined_parameters = parameters.join(", ");

    build_steps.push(format!("{circuit_name}.{gate_name}({joined_parameters})"));
}

#[expect(clippy::wildcard_enum_match_arm)]
fn get_qiskit_gate_name(gate_type: GateType) -> String {
    use GateType::*;

    match gate_type {
        Measure => "measure".to_owned(),
        _ => gate_type.to_string(),
    }
}
