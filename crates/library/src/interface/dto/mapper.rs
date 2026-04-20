use std::collections::HashSet;

use crate::{GateType, Graph, GraphBuilder, Position, dto::GateOperation};

/// Convert a list of gate operations into a `Graph`.
pub fn operations_to_graph(operations: &[GateOperation]) -> Graph {
    let mut builder = GraphBuilder::new();

    for operation in operations {
        builder.push_operation(&operation);
    }

    builder.build()
}

/// Convert a `Graph` into a list of `GateOperations`.
///
/// It assumes that the `Graph` is valid, and it will panic if it isn't.
/// Any `Graph` built with the `GraphBuilder` will always be valid.
/// To check that a manually built `Graph` is valid, use `Graph::validate`.
pub fn graph_to_operations(graph: Graph) -> Vec<GateOperation> {
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
                GateOperation::p(angle, row)
            }
            RX => {
                let angle = node.angle().expect("RX gate must have an angle");
                GateOperation::rx(angle, row)
            }
            RY => {
                let angle = node.angle().expect("RY gate must have an angle");
                GateOperation::ry(angle, row)
            }
            RZ => {
                let angle = node.angle().expect("RZ gate must have an angle");
                GateOperation::rz(angle, row)
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

                GateOperation::swap(row, other.row())
            }
            CH | CX | CY => {
                let edges = graph
                    .get_contextual_view(position)
                    .expect("Control gate must have edges");

                let is_target = edges.targets().is_empty();

                let (control, target) = if is_target {
                    (edges.controlled_by()[0].position(), position)
                } else {
                    (position, edges.targets()[0].position())
                };

                skipped.insert(control);
                skipped.insert(target);

                match node.r#type() {
                    CH => GateOperation::ch(control.row(), target.row()),
                    CX => GateOperation::cx(control.row(), target.row()),
                    CY => GateOperation::cy(control.row(), target.row()),
                    _ => unreachable!("Unexpected gate type"),
                }
            }
            CZ => {
                let edges = graph
                    .get_contextual_view(position)
                    .expect("CZ gate must have edges");

                let other = edges.works_with()[0].position();
                skipped.insert(other);

                GateOperation::cz(row, other.row())
            }
            CP => {
                let edges = graph
                    .get_contextual_view(position)
                    .expect("CP must have edges");

                let is_target = edges.targets().is_empty();

                let (control, target, angle) = if is_target {
                    let angle = node.angle().expect("CP gate must have an angle");

                    (edges.controlled_by()[0].position(), position, angle)
                } else {
                    let target_node = graph
                        .get_node(edges.targets()[0].position())
                        .expect("Target node missing");

                    let angle = target_node.angle().expect("CP gate must have an angle");

                    (position, edges.targets()[0].position(), angle)
                };

                skipped.insert(control);
                skipped.insert(target);

                GateOperation::cp(angle, control.row(), target.row())
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

                GateOperation::c_swap(control.row(), target1.row(), target2.row())
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

                GateOperation::ccx(control1.row(), control2.row(), target.row())
            }
            CCZ => {
                let edges = graph
                    .get_contextual_view(position)
                    .expect("CCZ gate must have edges");

                let other1 = edges.works_with()[0].position();
                let other2 = edges.works_with()[1].position();
                skipped.insert(other1);
                skipped.insert(other2);

                GateOperation::ccz(row, other1.row(), other2.row())
            }
        };

        gates.push(operation);
    }

    gates
}
