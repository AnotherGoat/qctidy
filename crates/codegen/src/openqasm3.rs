use std::fmt::Write;

use qsimplify::{
    AngleFormat, Circuit, GateOperation, GateType, Graph, PiFormat, formatter::format_angle,
};
use qsimplify_ports::CodeGenerationError;

pub(crate) fn generate(graph: &Graph) -> Result<String, CodeGenerationError> {
    let circuit = Circuit::from(graph);
    let mut headers = Vec::<String>::new();
    let mut build_steps = Vec::new();
    let mut has_sy_gate = false;

    headers.push("OPENQASM 3.0;".to_owned());
    headers.push("include \"stdgates.inc\";".to_owned());

    if !graph.is_empty() {
        build_steps.push(format!("qubit[{}] q;", graph.height()));

        if graph.bits() > 0 {
            build_steps.push(format!("bit[{}] c;", graph.bits()));
        }
    }

    for operation in circuit.operations() {
        generate_gate(&operation, &mut has_sy_gate, &mut build_steps);
    }

    let mut output = String::new();

    for header in &headers {
        writeln!(output, "{header}").expect("String should always be writable");
    }

    if has_sy_gate {
        output.push('\n');
        output.push_str("gate sy q {\n");
        output.push_str("    ry(-pi / 2) q;\n");
        output.push_str("}\n");
    }

    if !build_steps.is_empty() {
        output.push('\n');
    }

    for step in &build_steps {
        writeln!(output, "{step}").expect("String should always be writable");
    }

    Ok(output.trim_end().to_owned())
}

fn generate_gate(operation: &GateOperation, has_sy_gate: &mut bool, build_steps: &mut Vec<String>) {
    use GateOperation::*;

    match *operation {
        ID { .. } => {}
        H { qubit }
        | X { qubit }
        | Y { qubit }
        | Z { qubit }
        | S { qubit }
        | SDG { qubit }
        | SX { qubit }
        | T { qubit }
        | TDG { qubit } => {
            add_build_step(qubit, operation, build_steps);
        }
        P { angle, qubit } | RX { angle, qubit } | RY { angle, qubit } | RZ { angle, qubit } => {
            generate_rotation_gate(angle, qubit, operation, build_steps);
        }
        SY { qubit } => {
            *has_sy_gate = true;
            build_steps.push(format!("sy q[{qubit}];"));
        }
        Measure { qubit, bit } => {
            build_steps.push(format!("measure q[{qubit}] -> c[{bit}];"));
        }
        Swap { qubit1, qubit2 } | CZ { qubit1, qubit2 } => {
            add_build_step_with_parameters(&[qubit1, qubit2], operation, build_steps);
        }
        CH { control, target } | CX { control, target } | CY { control, target } => {
            add_build_step_with_parameters(&[control, target], operation, build_steps);
        }
        CP {
            angle,
            qubit1,
            qubit2,
        } => {
            generate_cp_gate(angle, qubit1, qubit2, operation, build_steps);
        }
        CSwap {
            control,
            target1,
            target2,
        } => {
            add_build_step_with_parameters(&[control, target1, target2], operation, build_steps);
        }
        CCX {
            control1,
            control2,
            target,
        } => {
            add_build_step_with_parameters(&[control1, control2, target], operation, build_steps);
        }
        CCZ {
            qubit1,
            qubit2,
            qubit3,
        } => {
            add_build_step_with_parameters(&[qubit1, qubit2, qubit3], operation, build_steps);
        }
    }
}

fn generate_rotation_gate(
    angle: f64,
    qubit: usize,
    operation: &GateOperation,
    build_steps: &mut Vec<String>,
) {
    let gate_name = get_openqasm3_gate_name(operation.r#type());

    let formatted = format_angle(angle, AngleFormat::Code, PiFormat::Lowercase);

    if formatted == "0" || formatted.contains("pi") {
        build_steps.push(format!("{gate_name}({formatted}) q[{qubit}];"));
        return;
    }

    build_steps.push(format!("{gate_name}({angle}) q[{qubit}];"));
}

fn generate_cp_gate(
    angle: f64,
    qubit1: usize,
    qubit2: usize,
    operation: &GateOperation,
    build_steps: &mut Vec<String>,
) {
    let gate_name = get_openqasm3_gate_name(operation.r#type());

    let formatted = format_angle(angle, AngleFormat::Code, PiFormat::Lowercase);

    if formatted == "0" || formatted.contains("pi") {
        build_steps.push(format!(
            "{gate_name}({formatted}) q[{qubit1}], q[{qubit2}];"
        ));
        return;
    }

    build_steps.push(format!("{gate_name}({angle}) q[{qubit1}], q[{qubit2}];"));
}

fn add_build_step(qubit: usize, operation: &GateOperation, build_steps: &mut Vec<String>) {
    let gate_name = get_openqasm3_gate_name(operation.r#type());

    build_steps.push(format!("{gate_name} q[{qubit}];"));
}

fn add_build_step_with_parameters(
    parameters: &[usize],
    operation: &GateOperation,
    build_steps: &mut Vec<String>,
) {
    let gate_name = get_openqasm3_gate_name(operation.r#type());
    let joined_parameters = parameters
        .iter()
        .map(|&parameter| format!("q[{parameter}]"))
        .collect::<Vec<_>>()
        .join(", ");

    build_steps.push(format!("{gate_name} {joined_parameters};"));
}

fn get_openqasm3_gate_name(gate_type: GateType) -> String {
    gate_type.to_string()
}
