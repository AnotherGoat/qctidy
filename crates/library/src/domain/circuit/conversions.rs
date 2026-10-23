use std::collections::HashSet;

use crate::{Circuit, GateOperation, GateType, Graph, Position};

/// Convert a `Graph` into a `Circuit`.
///
/// It assumes that the `Graph` is valid, and it will panic if it isn't.
/// Any `Graph` built with the `GraphBuilder` will always be valid.
/// To check that a manually built `Graph` is valid, use `Graph::validate`.
impl From<&Graph> for Circuit {
    fn from(graph: &Graph) -> Self {
        use GateType::*;

        let mut operations = Vec::new();
        let mut skipped: HashSet<Position> = HashSet::new();

        for node in graph.iter_nodes_ordered_by_time_step() {
            let position = node.position();

            if skipped.contains(&position) {
                continue;
            }

            let qubit = position.qubit();

            let operation = match node.r#type() {
                ID => GateOperation::id(qubit),
                H => GateOperation::h(qubit),
                X => GateOperation::x(qubit),
                Y => GateOperation::y(qubit),
                Z => GateOperation::z(qubit),
                P => {
                    let theta = node.theta().expect("P gate must have a theta angle");
                    GateOperation::try_p(theta, qubit).expect("Angle should be finite")
                }
                RX => {
                    let theta = node.theta().expect("RX gate must have a theta angle");
                    GateOperation::try_rx(theta, qubit).expect("Angle should be finite")
                }
                RY => {
                    let theta = node.theta().expect("RY gate must have a theta angle");
                    GateOperation::try_ry(theta, qubit).expect("Angle should be finite")
                }
                RZ => {
                    let phi = node.phi().expect("RZ gate must have a phi angle");
                    GateOperation::try_rz(phi, qubit).expect("Angle should be finite")
                }
                S => GateOperation::s(qubit),
                SDG => GateOperation::sdg(qubit),
                SX => GateOperation::sx(qubit),
                SY => GateOperation::sy(qubit),
                T => GateOperation::t(qubit),
                TDG => GateOperation::tdg(qubit),
                U => {
                    let theta = node.theta().expect("U gate must have a theta angle");
                    let phi = node.phi().expect("U gate must have a phi angle");
                    let lambda = node.lambda().expect("U gate must have a lambda angle");

                    GateOperation::try_u(theta, phi, lambda, qubit)
                        .expect("Angles should be finite")
                }
                Measure => {
                    let bit = node.bit().expect("Measure gate must have a bit");
                    GateOperation::measure(qubit, bit)
                }
                Swap => {
                    let edges = graph
                        .get_contextual_view(position)
                        .expect("Swap gate must have edges");

                    let other = edges
                        .swaps_with()
                        .as_ref()
                        .expect("Swap gate must have a partner")
                        .position();
                    skipped.insert(other);

                    GateOperation::try_swap(qubit, other.qubit())
                        .expect("Qubits should be distinct")
                }
                CH => {
                    let (control, target) = extract_control_and_target(graph, position);

                    skipped.insert(control);
                    skipped.insert(target);

                    GateOperation::try_ch(control.qubit(), target.qubit())
                        .expect("Qubits should be distinct")
                }
                CX => {
                    let (control, target) = extract_control_and_target(graph, position);

                    skipped.insert(control);
                    skipped.insert(target);

                    GateOperation::try_cx(control.qubit(), target.qubit())
                        .expect("Qubits should be distinct")
                }
                CY => {
                    let (control, target) = extract_control_and_target(graph, position);

                    skipped.insert(control);
                    skipped.insert(target);

                    GateOperation::try_cy(control.qubit(), target.qubit())
                        .expect("Qubits should be distinct")
                }
                CZ => {
                    let edges = graph
                        .get_contextual_view(position)
                        .expect("CZ gate must have edges");

                    let other = edges.works_with()[0].position();
                    skipped.insert(other);

                    GateOperation::try_cz(qubit, other.qubit()).expect("Qubits should be distinct")
                }
                CP => {
                    let edges = graph
                        .get_contextual_view(position)
                        .expect("CP must have edges");
                    let theta = node.theta().expect("CP gate must have a theta angle");

                    let other = edges.works_with()[0].position();
                    skipped.insert(other);

                    GateOperation::try_cp(theta, qubit, other.qubit())
                        .expect("Angle should be finite and qubits should be distinct")
                }
                CSwap => {
                    let edges = graph
                        .get_contextual_view(position)
                        .expect("CSwap must have edges");

                    let is_target = edges.targets().is_empty();

                    let (control, target1, target2) = if is_target {
                        (
                            edges.controlled_by()[0].position(),
                            position,
                            edges
                                .swaps_with()
                                .as_ref()
                                .expect("Missing swap partner")
                                .position(),
                        )
                    } else {
                        (
                            position,
                            edges.targets()[0].position(),
                            edges.targets()[1].position(),
                        )
                    };

                    skipped.insert(control);
                    skipped.insert(target1);
                    skipped.insert(target2);

                    GateOperation::try_c_swap(control.qubit(), target1.qubit(), target2.qubit())
                        .expect("Qubits should be distinct")
                }
                CCX => {
                    let edges = graph
                        .get_contextual_view(position)
                        .expect("CCX gate must have edges");

                    let is_target = edges.targets().is_empty();

                    let (control1, control2, target) = if is_target {
                        (
                            edges.controlled_by()[0].position(),
                            edges.controlled_by()[1].position(),
                            position,
                        )
                    } else {
                        (
                            position,
                            edges.works_with()[0].position(),
                            edges.targets()[0].position(),
                        )
                    };

                    skipped.insert(control1);
                    skipped.insert(control2);
                    skipped.insert(target);

                    GateOperation::try_ccx(control1.qubit(), control2.qubit(), target.qubit())
                        .expect("Qubits should be distinct")
                }
                CCZ => {
                    let edges = graph
                        .get_contextual_view(position)
                        .expect("CCZ gate must have edges");

                    let other1 = edges.works_with()[0].position();
                    let other2 = edges.works_with()[1].position();
                    skipped.insert(other1);
                    skipped.insert(other2);

                    GateOperation::try_ccz(qubit, other1.qubit(), other2.qubit())
                        .expect("Qubits should be distinct")
                }
            };

            operations.push(operation);
        }

        Self::new(graph.qubit_count(), operations)
    }
}

fn extract_control_and_target(graph: &Graph, position: Position) -> (Position, Position) {
    let edges = graph
        .get_contextual_view(position)
        .expect("Control gate must have edges");

    let is_target = edges.targets().is_empty();

    if is_target {
        (edges.controlled_by()[0].position(), position)
    } else {
        (position, edges.targets()[0].position())
    }
}
