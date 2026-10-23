use std::f64::consts::FRAC_PI_2;

use crate::{EdgeType, EdgeView, GateType, Graph, GraphBuilder, GraphError, Position};
use EdgeType::*;
use GateType::*;

#[test]
fn default_graph() {
    let graph = Graph::default();

    assert!(graph.is_empty());
    assert!(graph.time_step_count() == 0);
    assert!(graph.qubit_count() == 0);
}

#[test]
fn empty_sized_graph() {
    let size1 = Graph::new(1);

    assert!(size1.is_empty());
    assert!(size1.time_step_count() == 0);
    assert!(size1.qubit_count() == 1);

    let size2 = Graph::new(2);

    assert!(size2.is_empty());
    assert!(size2.time_step_count() == 0);
    assert!(size2.qubit_count() == 2);
}

#[test]
fn add_node() {
    let mut graph = Graph::default();
    let position = Position::new(0, 0);

    graph
        .add_node(ID, position, None, None, None, None)
        .unwrap();

    let node = graph.get_node(position).unwrap();
    assert_eq!(node.r#type(), ID);
    assert_eq!(node.position(), position);
}

#[test]
fn add_node_fails_if_it_exists() {
    let mut graph = Graph::default();
    let node_position = Position::new(0, 0);

    graph.replace_node(X, node_position, None, None, None, None);

    let result = graph.add_node(Y, node_position, None, None, None, None);

    match result {
        Err(GraphError::NodeAlreadyExists { position }) => assert_eq!(position, node_position),
        _ => panic!("Unexpected result: {result:?}"),
    }
}

#[test]
fn add_node_increases_qubit_count() {
    let mut graph = Graph::new(1);
    assert!(graph.qubit_count() == 1);

    graph
        .add_node(H, Position::new(0, 0), None, None, None, None)
        .unwrap();
    assert!(graph.qubit_count() == 1);

    graph
        .add_node(H, Position::new(1, 0), None, None, None, None)
        .unwrap();
    assert!(graph.qubit_count() == 2);

    graph
        .add_node(H, Position::new(2, 0), None, None, None, None)
        .unwrap();
    assert!(graph.qubit_count() == 3);
}

#[test]
fn replace_node_overwrites_existing_node() {
    let mut graph = Graph::default();
    let position = Position::new(0, 0);

    graph.replace_node(X, position, None, None, None, None);
    graph.replace_node(Y, position, None, None, None, None);

    let node = graph.get_node(position).unwrap();
    assert_eq!(node.r#type(), Y);
}

#[test]
fn replace_node_increases_qubit_count() {
    let mut graph = Graph::new(1);
    assert!(graph.qubit_count() == 1);

    graph.replace_node(Z, Position::new(0, 0), None, None, None, None);
    assert!(graph.qubit_count() == 1);

    graph.replace_node(Z, Position::new(0, 0), None, None, None, None);
    assert!(graph.qubit_count() == 1);

    graph.replace_node(Z, Position::new(1, 0), None, None, None, None);
    assert!(graph.qubit_count() == 2);
}

#[test]
fn empty_space_has_no_node() {
    let mut graph = Graph::default();

    graph.replace_node(X, Position::new(0, 2), None, None, None, None);

    assert!(!graph.has_node_at(Position::new(0, 0)));
    assert!(graph.get_node(Position::new(0, 0)).is_none());
}

#[test]
fn graph_time_step_count() {
    let mut graph = Graph::default();

    graph.replace_node(X, Position::new(0, 0), None, None, None, None);
    assert_eq!(graph.time_step_count(), 1);

    graph.replace_node(X, Position::new(0, 5), None, None, None, None);
    assert_eq!(graph.time_step_count(), 6);
}

#[test]
fn graph_qubit_count() {
    let mut graph = Graph::default();

    graph.replace_node(X, Position::new(0, 0), None, None, None, None);
    assert_eq!(graph.qubit_count(), 1);

    graph.replace_node(X, Position::new(5, 0), None, None, None, None);
    assert_eq!(graph.qubit_count(), 6);
}

#[test]
fn bit_count_counts_highest_measurement_bit() {
    let mut graph = Graph::default();

    graph.replace_node(Measure, Position::new(0, 0), None, None, None, Some(0));
    graph.replace_node(Measure, Position::new(1, 0), None, None, None, Some(3));

    assert_eq!(graph.bit_count(), 4);
}

#[test]
fn graph_equals() {
    let graph1 = GraphBuilder::default().push_x(0).push_x(1).build();
    let graph2 = GraphBuilder::default().push_x(0).push_x(1).build();

    assert_eq!(graph1, graph2);
}

#[test]
fn graph_is_not_equal() {
    let graph1 = GraphBuilder::default().push_x(0).push_x(1).build();
    let graph2 = GraphBuilder::default().push_x(0).push_y(1).build();

    assert_ne!(graph1, graph2);
}

#[test]
fn graph_is_equal_with_angles() {
    let mut graph1 = Graph::default();
    let mut graph2 = Graph::default();
    let first = Position::new(0, 0);
    let second = Position::new(0, 1);

    graph1.replace_node(RX, first, Some(0.0), None, None, None);
    graph1.replace_node(P, second, Some(FRAC_PI_2), None, None, None);

    graph2.replace_node(RX, first, Some(1e-8), None, None, None);
    graph2.replace_node(P, second, Some(1e-8 + FRAC_PI_2), None, None, None);

    assert_eq!(graph1, graph2);
}

#[test]
fn graph_is_equal_in_different_insertion_order() {
    let mut graph1 = Graph::default();
    let mut graph2 = Graph::default();
    let first = Position::new(0, 0);
    let second = Position::new(0, 1);

    graph1.replace_node(X, first, None, None, None, None);
    graph1.replace_node(Y, second, None, None, None, None);
    graph1.add_edge(Right, first, second).unwrap();

    graph2.replace_node(Y, second, None, None, None, None);
    graph2.replace_node(X, first, None, None, None, None);
    graph2.add_edge(Right, first, second).unwrap();

    assert_eq!(graph1, graph2);
}

#[test]
fn is_occupied_with_identity() {
    let mut graph = Graph::default();
    graph.replace_node(ID, Position::new(0, 0), None, None, None, None);
    graph.replace_node(X, Position::new(0, 1), None, None, None, None);

    assert!(graph.is_occupied(Position::new(0, 0)));
    assert!(graph.is_occupied(Position::new(0, 1)));
    assert!(!graph.is_occupied(Position::new(1, 0)));
}

#[test]
fn is_occupied() {
    let graph = GraphBuilder::default()
        .put_x(0, 1)
        .push_y(0)
        .push_z(1)
        .put_x(1, 2)
        .push_y(2)
        .push_z(2)
        .build();

    assert!(!graph.is_occupied(Position::new(0, 0)));
    assert!(graph.is_occupied(Position::new(0, 1)));
    assert!(graph.is_occupied(Position::new(0, 2)));
    assert!(graph.is_occupied(Position::new(1, 0)));
    assert!(!graph.is_occupied(Position::new(1, 1)));
    assert!(graph.is_occupied(Position::new(1, 2)));
    assert!(graph.is_occupied(Position::new(2, 0)));
    assert!(graph.is_occupied(Position::new(2, 1)));
    assert!(!graph.is_occupied(Position::new(2, 2)));
}

#[test]
fn has_node_at() {
    let mut graph = Graph::default();

    graph.replace_node(X, Position::new(0, 1), None, None, None, None);
    graph.replace_node(Y, Position::new(0, 2), None, None, None, None);
    graph.replace_node(Z, Position::new(1, 0), None, None, None, None);
    graph.replace_node(X, Position::new(1, 2), None, None, None, None);
    graph.replace_node(Y, Position::new(2, 0), None, None, None, None);
    graph.replace_node(Z, Position::new(2, 1), None, None, None, None);

    assert!(!graph.has_node_at(Position::new(0, 0)));
    assert!(graph.has_node_at(Position::new(0, 1)));
    assert!(graph.has_node_at(Position::new(0, 2)));
    assert!(graph.has_node_at(Position::new(1, 0)));
    assert!(!graph.has_node_at(Position::new(1, 1)));
    assert!(graph.has_node_at(Position::new(1, 2)));
    assert!(graph.has_node_at(Position::new(2, 0)));
    assert!(graph.has_node_at(Position::new(2, 1)));
    assert!(!graph.has_node_at(Position::new(2, 2)));
}

#[test]
fn doesnt_have_nodes_outside() {
    let graph = GraphBuilder::default().put_h(2, 2).build();

    assert!(!graph.has_node_at(Position::new(0, 3)));
    assert!(!graph.has_node_at(Position::new(3, 0)));
    assert!(!graph.has_node_at(Position::new(3, 3)));
}

#[test]
fn remove_nonexistent_node_is_noop() {
    let mut graph = Graph::default();

    graph.replace_node(X, Position::new(0, 0), None, None, None, None);

    graph.remove_node(Position::new(1, 1));

    assert_eq!(graph.node_count(), 1);
}

#[test]
fn remove_node_keeps_qubit_count() {
    let mut graph = GraphBuilder::default()
        .push_h(0)
        .push_x(1)
        .push_y(2)
        .push_z(3)
        .build();
    assert_eq!(graph.qubit_count(), 4);

    graph.remove_node(Position::new(0, 0));
    assert_eq!(graph.qubit_count(), 4);

    graph.remove_node(Position::new(1, 0));
    assert_eq!(graph.qubit_count(), 4);

    graph.remove_node(Position::new(3, 0));
    assert_eq!(graph.qubit_count(), 4);

    graph.remove_node(Position::new(2, 0));
    assert_eq!(graph.qubit_count(), 4);
}

#[test]
fn remove_node_removes_associated_edges() {
    let mut graph = Graph::default();

    let first = Position::new(0, 0);
    let second = Position::new(0, 1);

    graph.replace_node(X, first, None, None, None, None);
    graph.replace_node(Y, second, None, None, None, None);

    graph.add_edge(Right, first, second).unwrap();

    graph.remove_node(second);

    assert!(!graph.has_node_at(second));
    assert_eq!(graph.iter_edges_from(first).count(), 0);
}

#[test]
fn removing_middle_node_reconnects_neighbors() {
    let mut graph = Graph::default();

    let first = Position::new(0, 0);
    let second = Position::new(0, 1);
    let third = Position::new(0, 2);

    graph.replace_node(X, first, None, None, None, None);
    graph.replace_node(Y, second, None, None, None, None);
    graph.replace_node(Z, third, None, None, None, None);

    graph.add_edge(Right, first, second).unwrap();

    graph.add_edge(Right, second, third).unwrap();

    graph.remove_node(second);

    assert!(!graph.has_node_at(second));

    assert!(graph.iter_edges_unique().any(|edge| edge
        == EdgeView::new(
            Right,
            graph.get_node(first).unwrap(),
            graph.get_node(third).unwrap()
        )));
}

#[test]
fn move_nonexistent_node_fails() {
    let mut graph = Graph::default();
    let node_position = Position::new(0, 3);

    graph.replace_node(X, Position::new(0, 1), None, None, None, None);
    graph.replace_node(Y, Position::new(0, 2), None, None, None, None);

    let result = graph.move_node(node_position, Position::new(1, 3));

    match result {
        Err(GraphError::NodeNotFound { position }) => assert_eq!(position, node_position),
        _ => panic!("Unexpected result: {result:?}"),
    }
}

#[test]
fn null_move() {
    let mut graph = Graph::default();

    graph.replace_node(H, Position::new(0, 0), None, None, None, None);

    let result = graph.move_node(Position::new(0, 0), Position::new(0, 0));

    assert!(matches!(result, Err(GraphError::NullMove)));
}

#[test]
fn move_node() {
    let mut graph = Graph::default();

    graph.replace_node(X, Position::new(0, 1), None, None, None, None);
    graph.replace_node(Y, Position::new(1, 2), None, None, None, None);
    graph.replace_node(Z, Position::new(0, 3), None, None, None, None);

    graph
        .move_node(Position::new(0, 1), Position::new(1, 1))
        .unwrap();

    assert!(!graph.has_node_at(Position::new(0, 1)));

    let node = graph.get_node(Position::new(1, 1)).unwrap();
    assert_eq!(node.r#type(), X);
    assert_eq!(node.position(), Position::new(1, 1));
}

#[test]
fn move_node_increases_qubit_count_if_needed() {
    let mut graph = Graph::new(1);
    assert!(graph.qubit_count() == 1);

    graph.replace_node(T, Position::new(0, 0), None, None, None, None);
    graph
        .move_node(Position::new(0, 0), Position::new(1, 0))
        .unwrap();
    assert!(graph.qubit_count() == 2);

    graph
        .move_node(Position::new(1, 0), Position::new(2, 0))
        .unwrap();
    assert!(graph.qubit_count() == 3);
}

#[test]
fn move_node_doesnt_decrease_qubit_count() {
    let mut graph = Graph::new(3);
    assert!(graph.qubit_count() == 3);

    graph.replace_node(TDG, Position::new(2, 0), None, None, None, None);
    graph
        .move_node(Position::new(2, 0), Position::new(0, 0))
        .unwrap();
    assert!(graph.qubit_count() == 3);
}

#[test]
fn move_node_overwrites_destination() {
    let mut graph = Graph::default();

    let start = Position::new(0, 0);
    let end = Position::new(0, 1);

    graph.replace_node(X, start, None, None, None, None);
    graph.replace_node(Y, end, None, None, None, None);

    graph.move_node(start, end).unwrap();

    let node = graph.get_node(end).unwrap();
    assert_eq!(node.r#type(), X);
}

#[test]
fn move_node_preserves_semantic_edges() {
    let mut graph = Graph::default();

    graph.replace_node(H, Position::new(0, 1), None, None, None, None);
    graph.replace_node(Y, Position::new(1, 1), None, None, None, None);
    graph.replace_node(CX, Position::new(0, 0), None, None, None, None);
    graph.replace_node(CX, Position::new(1, 0), None, None, None, None);

    graph
        .add_edge(Right, Position::new(0, 0), Position::new(0, 1))
        .unwrap();
    graph
        .add_edge(Targets, Position::new(0, 0), Position::new(1, 0))
        .unwrap();

    graph
        .move_node(Position::new(0, 0), Position::new(4, 0))
        .unwrap();

    let hadamard = graph.get_node(Position::new(0, 1)).unwrap();
    let cx_controller = graph.get_node(Position::new(4, 0)).unwrap();
    let cx_target = graph.get_node(Position::new(1, 0)).unwrap();

    let edges: Vec<_> = graph.iter_edges_unique().collect();

    // The semantic `Targets` edge moves with the node.
    assert!(edges.contains(&EdgeView::new(Targets, cx_controller, cx_target)));

    // The time-step-order `Right` edge is recomputed for the new position, so it must not
    // follow the node across rows.
    assert!(!edges.contains(&EdgeView::new(Right, cx_controller, hadamard)));
}

#[test]
fn add_edge_fails_when_missing_nodes() {
    let mut graph = Graph::default();
    let start = Position::new(0, 0);
    let end = Position::new(0, 1);

    graph.replace_node(X, start, None, None, None, None);

    let result = graph.add_edge(Right, start, end);

    match result {
        Err(GraphError::NodeNotFound { position }) => assert_eq!(position, end),
        _ => panic!("Unexpected result: {result:?}"),
    }
}

#[test]
fn add_edge_is_idempotent() {
    let mut graph = Graph::default();

    let start = Position::new(0, 0);
    let end = Position::new(0, 1);

    graph.replace_node(X, start, None, None, None, None);
    graph.replace_node(X, end, None, None, None, None);

    graph.add_edge(Right, start, end).unwrap();
    assert_eq!(graph.iter_edges_from(start).count(), 1);

    graph.add_edge(Right, start, end).unwrap();
    assert_eq!(graph.iter_edges_from(start).count(), 1);
}

#[test]
fn add_bidirectional_edge_creates_two_edges() {
    let mut graph = Graph::default();

    let first = Position::new(0, 0);
    let second = Position::new(0, 1);

    graph.replace_node(X, first, None, None, None, None);
    graph.replace_node(Y, second, None, None, None, None);

    graph.add_edge(WorksWith, first, second).unwrap();

    let edges: Vec<_> = graph.iter_edges().collect();

    assert!(edges.contains(&EdgeView::new(
        WorksWith,
        graph.get_node(first).unwrap(),
        graph.get_node(second).unwrap()
    )));
    assert!(edges.contains(&EdgeView::new(
        WorksWith,
        graph.get_node(second).unwrap(),
        graph.get_node(first).unwrap()
    )));
}

#[test]
fn connect_qubit_neighbors_fails_if_node_is_missing() {
    let mut graph = Graph::default();
    let node_position = Position::new(0, 0);

    let result = graph.connect_qubit_neighbors(node_position);

    match result {
        Err(GraphError::NodeNotFound { position }) => assert_eq!(position, node_position),
        _ => panic!("Unexpected result: {result:?}"),
    }
}

#[test]
fn connect_qubit_neighbors_relinks_correctly() {
    let mut graph = Graph::default();

    let start = Position::new(0, 0);
    let middle = Position::new(0, 1);
    let end = Position::new(0, 2);

    graph.replace_node(X, start, None, None, None, None);
    graph.replace_node(Y, middle, None, None, None, None);
    graph.replace_node(Z, end, None, None, None, None);

    graph.connect_qubit_neighbors(middle).unwrap();

    let edges: Vec<_> = graph.iter_edges_unique().collect();

    assert!(edges.contains(&EdgeView::new(
        Right,
        graph.get_node(start).unwrap(),
        graph.get_node(middle).unwrap()
    )));
    assert!(edges.contains(&EdgeView::new(
        Right,
        graph.get_node(middle).unwrap(),
        graph.get_node(end).unwrap()
    )));
}

#[test]
fn connect_qubit_neighbors_removes_direct_connection() {
    let mut graph = Graph::default();

    let first = Position::new(0, 0);
    let second = Position::new(0, 1);
    let third = Position::new(0, 2);

    graph.replace_node(X, first, None, None, None, None);
    graph.replace_node(Y, second, None, None, None, None);
    graph.replace_node(Z, third, None, None, None, None);

    graph.add_edge(Right, first, third).unwrap();

    graph.connect_qubit_neighbors(second).unwrap();

    assert!(!graph.iter_edges_unique().any(|edge| edge
        == EdgeView::new(
            Right,
            graph.get_node(first).unwrap(),
            graph.get_node(third).unwrap()
        )));
}

#[test]
fn remove_edge_removes_outgoing_and_incoming_entries() {
    let mut graph = Graph::default();

    let first = Position::new(0, 0);
    let second = Position::new(0, 1);

    graph.replace_node(X, first, None, None, None, None);
    graph.replace_node(Y, second, None, None, None, None);

    graph.add_edge(Right, first, second).unwrap();

    graph.remove_edge(Right, first, second);

    assert_eq!(graph.iter_edges_from(first).count(), 0);
}

#[test]
fn next_in_qubit_returns_correct_node() {
    let mut graph = Graph::default();
    let position = Position::new(0, 0);
    let next_position = Position::new(0, 2);

    graph.replace_node(X, position, None, None, None, None);
    graph.replace_node(Y, next_position, None, None, None, None);

    let next = graph.next_in_qubit(position);

    assert_eq!(next, Some(next_position));
}

#[test]
fn next_in_qubit_none_if_no_next_exists() {
    let mut graph = Graph::default();

    let position = Position::new(0, 0);
    graph.replace_node(X, position, None, None, None, None);

    assert_eq!(graph.next_in_qubit(position), None);
}

#[test]
fn previous_in_qubit_returns_correct_node() {
    let mut graph = Graph::default();
    let position = Position::new(0, 2);
    let previous_position = Position::new(0, 0);

    graph.replace_node(X, previous_position, None, None, None, None);
    graph.replace_node(Y, position, None, None, None, None);

    let previous = graph.previous_in_qubit(position);

    assert_eq!(previous, Some(previous_position));
}

#[test]
fn previous_in_qubit_none_if_no_previous_exists() {
    let mut graph = Graph::default();

    let position = Position::new(0, 0);
    graph.replace_node(X, position, None, None, None, None);

    assert_eq!(graph.previous_in_qubit(position), None);
}

#[test]
fn clear_removes_everything() {
    let mut graph = GraphBuilder::default().push_x(0).push_y(1).build();

    graph.clear();

    assert!(graph.is_empty());
    assert_eq!(graph.qubit_count(), 0);
    assert_eq!(graph.iter_edges_unique().count(), 0);
}

#[test]
fn clear_edges_keeps_nodes() {
    let mut graph = Graph::default();

    let first = Position::new(0, 0);
    let second = Position::new(0, 1);

    graph.replace_node(X, first, None, None, None, None);
    graph.replace_node(Y, second, None, None, None, None);

    graph.add_edge(Right, first, second).unwrap();

    graph.clear_edges();

    assert!(graph.has_node_at(first));
    assert!(graph.has_node_at(second));
    assert_eq!(graph.iter_edges_unique().count(), 0);
}

#[test]
fn get_node_and_edges_collects_edges_correctly() {
    let mut graph = Graph::default();

    let first = Position::new(0, 0);
    let second = Position::new(0, 1);

    graph.replace_node(X, first, None, None, None, None);
    graph.replace_node(Y, second, None, None, None, None);

    graph.add_edge(Right, first, second).unwrap();

    let left_view = graph.get_contextual_view(first).unwrap();

    assert!(left_view.right().is_some());
    assert!(left_view.left().is_none());

    let right_view = graph.get_contextual_view(second).unwrap();

    assert!(right_view.right().is_none());
    assert!(right_view.left().is_some());
}

#[test]
fn graph_to_circuit_includes_identity() {
    use crate::Circuit;

    let graph = GraphBuilder::default().push_id(0).push_x(0).build();
    let circuit = Circuit::from(&graph);

    assert_eq!(circuit.operations().len(), 2);
    assert_eq!(circuit.operations()[0].r#type(), ID);
    assert_eq!(circuit.operations()[1].r#type(), X);
}

#[test]
fn circuit_to_graph_round_trip_with_id() {
    use crate::{Circuit, GateOperation};

    let circuit = Circuit::new(1, vec![GateOperation::id(0), GateOperation::x(0)]);
    let graph = Graph::from(&circuit);

    assert_eq!(graph.qubit_count(), 1);
    assert_eq!(graph.iter_nodes().count(), 2);
    assert!(graph.is_occupied(Position::new(0, 0)));
    assert!(graph.is_occupied(Position::new(0, 1)));

    let first = graph.get_node(Position::new(0, 0)).unwrap();
    let second = graph.get_node(Position::new(0, 1)).unwrap();

    assert_eq!(first.r#type(), ID);
    assert_eq!(second.r#type(), X);
}

#[test]
fn move_node_updates_qubit_neighbors() {
    let mut graph = Graph::default();
    graph.replace_node(X, Position::new(1, 0), None, None, None, None);
    graph.replace_node(Y, Position::new(1, 2), None, None, None, None);

    // Qubit 1 has `X -> Y`. Moving X out must not leave a cross-qubit `Right` edge.
    graph
        .move_node(Position::new(1, 0), Position::new(0, 0))
        .unwrap();

    let mut expected = Graph::default();
    expected.replace_node(X, Position::new(0, 0), None, None, None, None);
    expected.replace_node(Y, Position::new(1, 2), None, None, None, None);

    assert_eq!(graph, expected);
}

#[test]
fn move_node_connects_to_destination_qubit_neighbors() {
    let mut graph = Graph::default();
    graph.replace_node(X, Position::new(0, 0), None, None, None, None);
    graph.replace_node(Z, Position::new(0, 3), None, None, None, None);
    graph.replace_node(Y, Position::new(1, 1), None, None, None, None);

    // Inserting Y between X and Z must connect `X -> Y -> Z`.
    graph
        .move_node(Position::new(1, 1), Position::new(0, 2))
        .unwrap();

    let mut expected = Graph::new(2);
    expected.replace_node(X, Position::new(0, 0), None, None, None, None);
    expected.replace_node(Y, Position::new(0, 2), None, None, None, None);
    expected.replace_node(Z, Position::new(0, 3), None, None, None, None);

    assert_eq!(graph, expected);
}

#[test]
fn validate_rejects_invalid_time_step_order_edges() {
    let mut graph = Graph::default();
    graph.replace_node(X, Position::new(0, 0), None, None, None, None);
    graph.replace_node(Y, Position::new(0, 2), None, None, None, None);
    graph.replace_node(Z, Position::new(1, 0), None, None, None, None);

    assert!(graph.validate().is_ok());

    // A `Right` edge must connect qubit neighbors; this one crosses rows.
    graph
        .add_edge(Right, Position::new(0, 0), Position::new(1, 0))
        .unwrap();

    assert!(matches!(
        graph.validate(),
        Err(GraphError::InvalidTimeStepOrder { .. })
    ));
}

#[test]
fn counts_operations_gates_and_measurements() {
    let graph = GraphBuilder::new(2)
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_x(0)
        .push_measure(0, 0)
        .build();

    assert_eq!(graph.operation_count(), 3);
    assert_eq!(graph.gate_count(), 2);
    assert_eq!(graph.measurement_count(), 1);
}
