use std::f64::consts::FRAC_PI_2;

use super::quantum_graph::*;
use crate::{
    domain::{EdgeType, GateType, GraphBuilder, Position},
    view::GraphEdgeView,
};
use EdgeType::*;
use GateType::*;

#[test]
fn add_node() {
    let mut graph = QuantumGraph::new();
    let position = Position::new(0, 0);

    graph.add_node(ID, position, None, None).unwrap();

    let node = graph.get_node_view(position).unwrap();
    assert_eq!(node.r#type(), ID);
    assert_eq!(node.position(), position);
}

#[test]
#[allow(unused_variables)]
fn add_node_fails_if_it_exists() {
    let mut graph = QuantumGraph::new();
    let position = Position::new(0, 0);

    graph.replace_node(X, position, None, None);

    let result = graph.add_node(Y, position, None, None);

    assert!(matches!(
        result,
        Err(GraphError::NodeAlreadyExists { position })
    ));
}

#[test]
fn replace_node_overwrites_existing_node() {
    let mut graph = QuantumGraph::new();
    let position = Position::new(0, 0);

    graph.replace_node(X, position, None, None);
    graph.replace_node(Y, position, None, None);

    let node = graph.get_node_view(position).unwrap();
    assert_eq!(node.r#type(), Y);
}

#[test]
fn empty_space_has_no_node() {
    let mut graph = QuantumGraph::new();

    graph.replace_node(X, Position::new(0, 2), None, None);

    assert!(!graph.has_node_at(Position::new(0, 0)));
    assert!(graph.get_node_view(Position::new(0, 0)).is_none());
}

#[test]
fn graph_width() {
    let mut graph = QuantumGraph::new();

    graph.replace_node(X, Position::new(0, 0), None, None);
    assert_eq!(graph.width(), 1);

    graph.replace_node(X, Position::new(0, 5), None, None);
    assert_eq!(graph.width(), 6);
}

#[test]
fn graph_height() {
    let mut graph = QuantumGraph::new();

    graph.replace_node(X, Position::new(0, 0), None, None);
    assert_eq!(graph.height(), 1);

    graph.replace_node(X, Position::new(5, 0), None, None);
    assert_eq!(graph.height(), 6);
}

#[test]
fn bits_counts_highest_measurement_bit() {
    let mut graph = QuantumGraph::new();

    graph.replace_node(Measure, Position::new(0, 0), None, Some(0));
    graph.replace_node(Measure, Position::new(1, 0), None, Some(3));

    assert_eq!(graph.bits(), 4);
}

#[test]
fn empty_dimensions() {
    let graph = QuantumGraph::new();

    assert_eq!(graph.width(), 0);
    assert_eq!(graph.height(), 0);
}

#[test]
fn graph_equals() {
    let graph1 = GraphBuilder::new().push_x(0).push_x(1).build();
    let graph2 = GraphBuilder::new().push_x(0).push_x(1).build();

    assert_eq!(graph1, graph2);
}

#[test]
fn graph_is_not_equal() {
    let graph1 = GraphBuilder::new().push_x(0).push_x(1).build();
    let graph2 = GraphBuilder::new().push_x(0).push_y(1).build();

    assert_ne!(graph1, graph2);
}

#[test]
fn graph_is_equal_with_angles() {
    let mut graph1 = QuantumGraph::new();
    let mut graph2 = QuantumGraph::new();
    let first = Position::new(0, 0);
    let second = Position::new(0, 1);

    graph1.replace_node(RX, first, Some(0.0), None);
    graph1.replace_node(P, second, Some(FRAC_PI_2), None);

    graph2.replace_node(RX, first, Some(1e-8), None);
    graph2.replace_node(P, second, Some(1e-8 + FRAC_PI_2), None);

    assert_eq!(graph1, graph2);
}

#[test]
fn graph_is_equal_in_different_order() {
    let mut graph1 = QuantumGraph::new();
    let mut graph2 = QuantumGraph::new();
    let first = Position::new(0, 0);
    let second = Position::new(0, 1);

    graph1.replace_node(X, first, None, None);
    graph1.replace_node(Y, second, None, None);
    graph1.add_edge(Right, first, second).unwrap();
    graph1.add_edge(Left, second, first).unwrap();

    graph2.replace_node(Y, second, None, None);
    graph2.replace_node(X, first, None, None);
    graph2.add_edge(Left, second, first).unwrap();
    graph2.add_edge(Right, first, second).unwrap();

    assert_eq!(graph1, graph2);
}

#[test]
fn is_occupied() {
    let graph = GraphBuilder::new()
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
    let mut graph = QuantumGraph::new();

    graph.replace_node(X, Position::new(0, 1), None, None);
    graph.replace_node(Y, Position::new(0, 2), None, None);
    graph.replace_node(Z, Position::new(1, 0), None, None);
    graph.replace_node(X, Position::new(1, 2), None, None);
    graph.replace_node(Y, Position::new(2, 0), None, None);
    graph.replace_node(Z, Position::new(2, 1), None, None);

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
    let graph = GraphBuilder::new().put_h(2, 2).build();

    assert!(!graph.has_node_at(Position::new(0, 3)));
    assert!(!graph.has_node_at(Position::new(3, 0)));
    assert!(!graph.has_node_at(Position::new(3, 3)));
}

#[test]
fn remove_nonexistent_node_is_noop() {
    let mut graph = QuantumGraph::new();

    graph.replace_node(X, Position::new(0, 0), None, None);

    graph.remove_node(Position::new(1, 1));

    assert_eq!(graph.size(), 1);
}

#[test]
fn remove_node_removes_associated_edges() {
    let mut graph = QuantumGraph::new();

    let first = Position::new(0, 0);
    let second = Position::new(0, 1);

    graph.replace_node(X, first, None, None);
    graph.replace_node(Y, second, None, None);

    graph.add_edge(Right, first, second).unwrap();
    graph.add_edge(Left, second, first).unwrap();

    graph.remove_node(first);

    assert!(!graph.has_node_at(first));
    assert_eq!(graph.iter_node_edges(second).count(), 0);
}

#[test]
fn removing_middle_node_reconnects_neighbors() {
    let mut graph = QuantumGraph::new();

    let first = Position::new(0, 0);
    let second = Position::new(0, 1);
    let third = Position::new(0, 2);

    graph.replace_node(X, first, None, None);
    graph.replace_node(Y, second, None, None);
    graph.replace_node(Z, third, None, None);

    graph.add_edge(Right, first, second).unwrap();
    graph.add_edge(Left, second, first).unwrap();

    graph.add_edge(Right, second, third).unwrap();
    graph.add_edge(Left, third, second).unwrap();

    graph.remove_node(second);

    assert!(!graph.has_node_at(second));

    let edges = graph.edges();

    assert!(edges.contains(&GraphEdgeView::new(
        Right,
        graph.get_node_view(first).unwrap(),
        graph.get_node_view(third).unwrap()
    )));

    assert!(edges.contains(&GraphEdgeView::new(
        Left,
        graph.get_node_view(third).unwrap(),
        graph.get_node_view(first).unwrap()
    )));
}

#[test]
#[allow(unused_variables)]
fn move_nonexistent_node_fails() {
    let mut graph = QuantumGraph::new();
    let position = Position::new(0, 3);

    graph.replace_node(X, Position::new(0, 1), None, None);
    graph.replace_node(Y, Position::new(0, 2), None, None);

    let result = graph.move_node(position, Position::new(1, 3));

    assert!(matches!(result, Err(GraphError::NodeNotFound { position })));
}

#[test]
fn null_move() {
    let mut graph = QuantumGraph::new();

    graph.replace_node(H, Position::new(0, 0), None, None);

    let result = graph.move_node(Position::new(0, 0), Position::new(0, 0));

    assert!(matches!(result, Err(GraphError::NullMove)));
}

#[test]
fn move_node() {
    let mut graph = QuantumGraph::new();

    graph.replace_node(X, Position::new(0, 1), None, None);
    graph.replace_node(Y, Position::new(1, 2), None, None);
    graph.replace_node(Z, Position::new(0, 3), None, None);

    graph
        .move_node(Position::new(0, 1), Position::new(1, 1))
        .unwrap();

    assert!(!graph.has_node_at(Position::new(0, 1)));

    let node = graph.get_node_view(Position::new(1, 1)).unwrap();
    assert_eq!(node.r#type(), X);
    assert_eq!(node.position(), Position::new(1, 1));
}

#[test]
fn move_node_overwrites_destination() {
    let mut graph = QuantumGraph::new();

    let start = Position::new(0, 0);
    let end = Position::new(0, 1);

    graph.replace_node(X, start, None, None);
    graph.replace_node(Y, end, None, None);

    graph.move_node(start, end).unwrap();

    let node = graph.get_node_view(end).unwrap();
    assert_eq!(node.r#type(), X);
}

#[test]
fn move_node_preserves_edges() {
    let mut graph = QuantumGraph::new();

    graph.replace_node(H, Position::new(0, 1), None, None);
    graph.replace_node(Y, Position::new(1, 1), None, None);
    graph.replace_node(CX, Position::new(0, 0), None, None);
    graph.replace_node(CX, Position::new(1, 0), None, None);

    graph
        .add_edge(Right, Position::new(0, 0), Position::new(0, 1))
        .unwrap();
    graph
        .add_edge(Left, Position::new(0, 1), Position::new(0, 0))
        .unwrap();
    graph
        .add_edge(Targets, Position::new(0, 0), Position::new(1, 0))
        .unwrap();
    graph
        .add_edge(ControlledBy, Position::new(1, 0), Position::new(0, 0))
        .unwrap();

    graph
        .move_node(Position::new(0, 0), Position::new(4, 0))
        .unwrap();

    let hadamard = graph.get_node_view(Position::new(0, 1)).unwrap();
    let cx_controller = graph.get_node_view(Position::new(4, 0)).unwrap();
    let cx_target = graph.get_node_view(Position::new(1, 0)).unwrap();

    let edges = graph.edges();

    assert!(edges.contains(&GraphEdgeView::new(
        Right,
        cx_controller.clone(),
        hadamard.clone()
    )));
    assert!(edges.contains(&GraphEdgeView::new(
        Left,
        hadamard.clone(),
        cx_controller.clone()
    )));
    assert!(edges.contains(&GraphEdgeView::new(
        Targets,
        cx_controller.clone(),
        cx_target.clone()
    )));
    assert!(edges.contains(&GraphEdgeView::new(
        ControlledBy,
        cx_target.clone(),
        cx_controller.clone()
    )));
}

#[test]
#[allow(unused_variables)]
fn add_edge_fails_when_missing_nodes() {
    let mut graph = QuantumGraph::new();
    let start = Position::new(0, 0);
    let end = Position::new(0, 1);

    graph.replace_node(X, start, None, None);

    let result = graph.add_edge(Right, start, end);

    assert!(matches!(
        result,
        Err(GraphError::NodeNotFound { position: end })
    ));
}

#[test]
fn add_edge_is_idempotent() {
    let mut graph = QuantumGraph::new();

    let start = Position::new(0, 0);
    let end = Position::new(0, 1);

    graph.replace_node(X, start, None, None);
    graph.replace_node(X, end, None, None);

    graph.add_edge(Right, start, end).unwrap();
    assert_eq!(graph.iter_node_edges(start).count(), 1);

    graph.add_edge(Right, start, end).unwrap();
    assert_eq!(graph.iter_node_edges(start).count(), 1);
}

#[test]
fn add_bidirectional_edge_creates_two_edges() {
    let mut graph = QuantumGraph::new();

    let first = Position::new(0, 0);
    let second = Position::new(0, 1);

    graph.replace_node(X, first, None, None);
    graph.replace_node(Y, second, None, None);

    graph
        .add_bidirectional_edge(WorksWith, first, second)
        .unwrap();

    let edges = graph.edges();

    assert!(edges.contains(&GraphEdgeView::new(
        WorksWith,
        graph.get_node_view(first).unwrap(),
        graph.get_node_view(second).unwrap()
    )));

    assert!(edges.contains(&GraphEdgeView::new(
        WorksWith,
        graph.get_node_view(second).unwrap(),
        graph.get_node_view(first).unwrap()
    )));
}

#[test]
#[allow(unused_variables)]
fn connect_row_neighbors_fails_if_node_is_missing() {
    let mut graph = QuantumGraph::new();
    let position = Position::new(0, 0);

    let result = graph.connect_row_neighbors(position);

    assert!(matches!(result, Err(GraphError::NodeNotFound { position })));
}

#[test]
fn connect_row_neighbors_relinks_correctly() {
    let mut graph = QuantumGraph::new();

    let start = Position::new(0, 0);
    let middle = Position::new(0, 1);
    let end = Position::new(0, 2);

    graph.replace_node(X, start, None, None);
    graph.replace_node(Y, middle, None, None);
    graph.replace_node(Z, end, None, None);

    graph.connect_row_neighbors(middle).unwrap();

    let edges = graph.edges();

    assert!(edges.contains(&GraphEdgeView::new(
        Right,
        graph.get_node_view(start).unwrap(),
        graph.get_node_view(middle).unwrap()
    )));

    assert!(edges.contains(&GraphEdgeView::new(
        Right,
        graph.get_node_view(middle).unwrap(),
        graph.get_node_view(end).unwrap()
    )));
}

#[test]
fn connect_row_neighbors_removes_direct_connection() {
    let mut graph = QuantumGraph::new();

    let first = Position::new(0, 0);
    let second = Position::new(0, 1);
    let third = Position::new(0, 2);

    graph.replace_node(X, first, None, None);
    graph.replace_node(Y, second, None, None);
    graph.replace_node(Z, third, None, None);

    graph.add_edge(Right, first, third).unwrap();

    graph.connect_row_neighbors(second).unwrap();

    let edges = graph.edges();

    assert!(!edges.contains(&GraphEdgeView::new(
        Right,
        graph.get_node_view(first).unwrap(),
        graph.get_node_view(third).unwrap()
    )));
}

#[test]
fn remove_edge_removes_outgoing_and_incoming_entries() {
    let mut graph = QuantumGraph::new();

    let first = Position::new(0, 0);
    let second = Position::new(0, 1);

    graph.replace_node(X, first, None, None);
    graph.replace_node(Y, second, None, None);

    graph.add_edge(Right, first, second).unwrap();

    graph.remove_edge(Right, first, second);

    assert_eq!(graph.iter_node_edges(first).count(), 0);
}

#[test]
fn next_in_row_returns_correct_node() {
    let mut graph = QuantumGraph::new();
    let position = Position::new(0, 0);
    let next_position = Position::new(0, 2);

    graph.replace_node(X, position, None, None);
    graph.replace_node(Y, next_position, None, None);

    let next = graph.next_in_row(position);

    assert_eq!(next, Some(next_position));
}

#[test]
fn next_in_row_none_if_no_next_exists() {
    let mut graph = QuantumGraph::new();

    let position = Position::new(0, 0);
    graph.replace_node(X, position, None, None);

    assert_eq!(graph.next_in_row(position), None);
}

#[test]
fn previous_in_row_returns_correct_node() {
    let mut graph = QuantumGraph::new();
    let position = Position::new(0, 2);
    let previous_position = Position::new(0, 0);

    graph.replace_node(X, previous_position, None, None);
    graph.replace_node(Y, position, None, None);

    let previous = graph.previous_in_row(position);

    assert_eq!(previous, Some(previous_position));
}

#[test]
fn previous_in_row_none_if_no_previous_exists() {
    let mut graph = QuantumGraph::new();

    let position = Position::new(0, 0);
    graph.replace_node(X, position, None, None);

    assert_eq!(graph.previous_in_row(position), None);
}

#[test]
fn clear_removes_everything() {
    let mut graph = GraphBuilder::new().push_x(0).push_y(1).build();

    graph.clear();

    assert!(graph.is_empty());
    assert_eq!(graph.edges().len(), 0);
}

#[test]
fn clear_edges_keeps_nodes() {
    let mut graph = QuantumGraph::new();

    let first = Position::new(0, 0);
    let second = Position::new(0, 1);

    graph.replace_node(X, first, None, None);
    graph.replace_node(Y, second, None, None);

    graph.add_edge(Right, first, second).unwrap();

    graph.clear_edges();

    assert!(graph.has_node_at(first));
    assert!(graph.has_node_at(second));
    assert_eq!(graph.edges().len(), 0);
}

#[test]
fn node_edge_view_collects_edges_correctly() {
    let mut graph = QuantumGraph::new();

    let first = Position::new(0, 0);
    let second = Position::new(0, 1);

    graph.replace_node(X, first, None, None);
    graph.replace_node(Y, second, None, None);

    graph.add_edge(Right, first, second).unwrap();
    graph.add_edge(Left, second, first).unwrap();

    let left_view = graph.node_edge_view(first).unwrap();

    assert!(left_view.right().is_some());
    assert!(left_view.left().is_none());

    let right_view = graph.node_edge_view(second).unwrap();

    assert!(right_view.right().is_none());
    assert!(right_view.left().is_some());
}
