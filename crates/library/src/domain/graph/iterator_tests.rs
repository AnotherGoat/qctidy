use crate::{GateType::*, Graph};
use crate::{GraphBuilder, Position};

#[test]
fn iter_positions_ordered_by_row() {
    let graph = GraphBuilder::default()
        .push_h(0)
        .push_h(1)
        .push_h(2)
        .push_x(0)
        .push_x(1)
        .build();

    let positions: Vec<_> = graph.iter_positions_ordered_by_row().collect();

    assert_eq!(positions.len(), 5);
    assert_eq!(positions[0], Position::new(0, 0));
    assert_eq!(positions[1], Position::new(0, 1));
    assert_eq!(positions[2], Position::new(1, 0));
    assert_eq!(positions[3], Position::new(1, 1));
    assert_eq!(positions[4], Position::new(2, 0));
}

#[test]
fn iter_positions_ordered_by_column() {
    let graph = GraphBuilder::default()
        .push_h(0)
        .push_h(1)
        .push_h(2)
        .push_x(0)
        .push_x(1)
        .build();

    let positions: Vec<_> = graph.iter_positions_ordered_by_column().collect();

    assert_eq!(positions.len(), 5);
    assert_eq!(positions[0], Position::new(0, 0));
    assert_eq!(positions[1], Position::new(1, 0));
    assert_eq!(positions[2], Position::new(2, 0));
    assert_eq!(positions[3], Position::new(0, 1));
    assert_eq!(positions[4], Position::new(1, 1));
}

#[test]
fn iter_positions_ordered_by_column_in_sparse_graph() {
    let mut graph = Graph::default();

    graph.replace_node(X, Position::new(2, 4), None, None);
    graph.replace_node(Y, Position::new(0, 2), None, None);
    graph.replace_node(Z, Position::new(1, 0), None, None);

    let positions: Vec<_> = graph.iter_positions_ordered_by_column().collect();
    assert_eq!(positions.len(), 3);
    assert_eq!(positions[0], Position::new(1, 0));
    assert_eq!(positions[1], Position::new(0, 2));
    assert_eq!(positions[2], Position::new(2, 4));
}

#[test]
#[expect(clippy::unwrap_used)]
fn iter_nodes_ordered_by_row() {
    let graph = GraphBuilder::default()
        .push_h(0)
        .push_h(1)
        .push_cx(0, 1)
        .unwrap()
        .build();

    let nodes: Vec<_> = graph.iter_nodes_ordered_by_row().collect();

    assert_eq!(nodes.len(), 4);
    assert_eq!(nodes[0].position(), Position::new(0, 0));
    assert_eq!(nodes[0].r#type(), H);
    assert_eq!(nodes[1].position(), Position::new(0, 1));
    assert_eq!(nodes[1].r#type(), CX);
    assert_eq!(nodes[2].position(), Position::new(1, 0));
    assert_eq!(nodes[2].r#type(), H);
    assert_eq!(nodes[3].position(), Position::new(1, 1));
    assert_eq!(nodes[3].r#type(), CX);
}

#[test]
fn iter_positions_ordered_by_row_in_sparse_graph() {
    let mut graph = Graph::default();

    graph.replace_node(X, Position::new(2, 4), None, None);
    graph.replace_node(Y, Position::new(0, 2), None, None);
    graph.replace_node(Z, Position::new(1, 0), None, None);

    let positions: Vec<_> = graph.iter_positions_ordered_by_row().collect();
    assert_eq!(positions.len(), 3);
    assert_eq!(positions[0], Position::new(0, 2));
    assert_eq!(positions[1], Position::new(1, 0));
    assert_eq!(positions[2], Position::new(2, 4));
}

#[test]
#[expect(clippy::unwrap_used)]
fn iter_nodes_ordered_by_column() {
    let graph = GraphBuilder::default()
        .push_h(0)
        .push_h(1)
        .push_cx(0, 1)
        .unwrap()
        .build();

    let nodes: Vec<_> = graph.iter_nodes_ordered_by_column().collect();

    assert_eq!(nodes.len(), 4);
    assert_eq!(nodes[0].position(), Position::new(0, 0));
    assert_eq!(nodes[0].r#type(), H);
    assert_eq!(nodes[1].position(), Position::new(1, 0));
    assert_eq!(nodes[1].r#type(), H);
    assert_eq!(nodes[2].position(), Position::new(0, 1));
    assert_eq!(nodes[2].r#type(), CX);
    assert_eq!(nodes[3].position(), Position::new(1, 1));
    assert_eq!(nodes[3].r#type(), CX);
}
