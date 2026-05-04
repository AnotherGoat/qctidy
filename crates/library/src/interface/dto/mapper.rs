use std::collections::HashSet;

use crate::{GateOperation, GateType, Graph, GraphBuilder, Position};

/// Convert a list of gate operations into a `Graph`.
pub fn operations_to_graph(operations: &[GateOperation]) -> Graph {
    GraphBuilder::new().push_operations(operations).build()
}

/// Convert a `Graph` into a list of `GateOperations`.
///
/// It assumes that the `Graph` is valid, and it will panic if it isn't.
/// Any `Graph` built with the `GraphBuilder` will always be valid.
/// To check that a manually built `Graph` is valid, use `Graph::validate`.
/// If the original `Graph` has any manually added `ID` gates, they will not be included.
#[must_use]
pub fn graph_to_operations(graph: &Graph) -> Vec<GateOperation> {
    use GateType::*;

    let mut gates = Vec::new();
    let mut skipped: HashSet<Position> = HashSet::new();

    for node in graph.iter_nodes_ordered_by_column() {
        let position = node.position();

        if skipped.contains(&position) {
            continue;
        }

        let row = position.row();

        let operation = match node.r#type() {
            ID => {
                continue;
            }
            H => GateOperation::h(row),
            X => GateOperation::x(row),
            Y => GateOperation::y(row),
            Z => GateOperation::z(row),
            P => {
                let angle = node.angle().expect("P gate must have an angle");
                GateOperation::try_p(angle, row).expect("Angle should be finite")
            }
            RX => {
                let angle = node.angle().expect("RX gate must have an angle");
                GateOperation::try_rx(angle, row).expect("Angle should be finite")
            }
            RY => {
                let angle = node.angle().expect("RY gate must have an angle");
                GateOperation::try_ry(angle, row).expect("Angle should be finite")
            }
            RZ => {
                let angle = node.angle().expect("RZ gate must have an angle");
                GateOperation::try_rz(angle, row).expect("Angle should be finite")
            }
            S => GateOperation::s(row),
            SDG => GateOperation::sdg(row),
            SX => GateOperation::sx(row),
            SY => GateOperation::sy(row),
            T => GateOperation::t(row),
            TDG => GateOperation::tdg(row),
            Measure => {
                let bit = node.bit().expect("Measure gate must have a bit");
                GateOperation::measure(row, bit)
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

                GateOperation::try_swap(row, other.row()).expect("Qubits should be distinct")
            }
            CH => {
                let (control, target) = extract_control_and_target(graph, position);

                skipped.insert(control);
                skipped.insert(target);

                GateOperation::try_ch(control.row(), target.row())
                    .expect("Qubits should be distinct")
            }
            CX => {
                let (control, target) = extract_control_and_target(graph, position);

                skipped.insert(control);
                skipped.insert(target);

                GateOperation::try_cx(control.row(), target.row())
                    .expect("Qubits should be distinct")
            }
            CY => {
                let (control, target) = extract_control_and_target(graph, position);

                skipped.insert(control);
                skipped.insert(target);

                GateOperation::try_cy(control.row(), target.row())
                    .expect("Qubits should be distinct")
            }
            CZ => {
                let edges = graph
                    .get_contextual_view(position)
                    .expect("CZ gate must have edges");

                let other = edges.works_with()[0].position();
                skipped.insert(other);

                GateOperation::try_cz(row, other.row()).expect("Qubits should be distinct")
            }
            CP => {
                let edges = graph
                    .get_contextual_view(position)
                    .expect("CP must have edges");
                let angle = node.angle().expect("CP gate must have an angle");

                let other = edges.works_with()[0].position();
                skipped.insert(other);

                GateOperation::try_cp(angle, row, other.row())
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

                GateOperation::try_c_swap(control.row(), target1.row(), target2.row())
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

                GateOperation::try_ccx(control1.row(), control2.row(), target.row())
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

                GateOperation::try_ccz(row, other1.row(), other2.row())
                    .expect("Qubits should be distinct")
            }
        };

        gates.push(operation);
    }

    gates
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
