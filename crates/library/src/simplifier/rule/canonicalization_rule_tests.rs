use std::collections::HashSet;

use crate::{
    Graph, GraphBuilder, Position, SimplificationRule, simplifier::rule::default::canonicalization,
};

#[test]
fn compact_rows_detects_gap() {
    let graph = GraphBuilder::new(3).push_x(0).push_x(2).build();

    let rule = canonicalization::compact_rows();
    let matches = rule.detect(&graph);

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].len(), 2);
    assert!(matches[0].contains(&Position::new(0, 0)));
    assert!(matches[0].contains(&Position::new(2, 0)));
}

#[test]
fn compact_rows_no_gap() {
    let graph = GraphBuilder::new(2).push_x(0).push_x(1).build();

    let rule = canonicalization::compact_rows();
    let matches = rule.detect(&graph);

    assert!(matches.is_empty());
}

#[test]
fn compact_rows_empty_graph() {
    let graph = Graph::default();

    let rule = canonicalization::compact_rows();
    let matches = rule.detect(&graph);

    assert!(matches.is_empty());
}

#[test]
fn compact_rows_no_first_row() {
    let graph = GraphBuilder::new(3).put_x(1, 0).put_x(2, 0).build();

    let rule = canonicalization::compact_rows();
    let matches = rule.detect(&graph);

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].len(), 2);
    assert!(matches[0].contains(&Position::new(1, 0)));
    assert!(matches[0].contains(&Position::new(2, 0)));
}

#[test]
fn compact_rows_empty_row_at_end_no_gap() {
    let graph = GraphBuilder::new(2).push_x(0).build();

    let rule = canonicalization::compact_rows();
    let matches = rule.detect(&graph);

    assert!(matches.is_empty());
}

#[test]
fn compact_columns_detects_gap() {
    let graph = GraphBuilder::new(1).put_x(0, 0).put_x(0, 2).build();

    let rule = canonicalization::compact_columns();
    let matches = rule.detect(&graph);

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].len(), 2);
    assert!(matches[0].contains(&Position::new(0, 0)));
    assert!(matches[0].contains(&Position::new(0, 2)));
}

#[test]
fn compact_columns_no_first_column() {
    let graph = GraphBuilder::new(1).put_x(0, 1).build();

    let rule = canonicalization::compact_columns();
    let matches = rule.detect(&graph);

    assert_eq!(matches.len(), 1);
    assert!(matches[0].contains(&Position::new(0, 1)));
}

#[test]
fn compact_columns_empty_column_after_remove() {
    let mut builder = GraphBuilder::new(1);
    builder.put_x(0, 0);
    builder.put_x(0, 1);
    builder.put_x(0, 2);
    let mut graph = builder.build();
    graph.remove_node(Position::new(0, 1));
    let rule = canonicalization::compact_columns();
    let matches = rule.detect(&graph);

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].len(), 2);
    assert!(matches[0].contains(&Position::new(0, 0)));
    assert!(matches[0].contains(&Position::new(0, 2)));
}

#[test]
fn compact_columns_no_gap() {
    let graph = GraphBuilder::default().push_x(0).build();

    let rule = canonicalization::compact_columns();
    let matches = rule.detect(&graph);

    assert!(matches.is_empty());
}

#[test]
fn compact_columns_empty_graph() {
    let graph = Graph::default();

    let rule = canonicalization::compact_columns();
    let matches = rule.detect(&graph);

    assert!(matches.is_empty());
}

#[test]
fn normalize_angles_detects_out_of_range() {
    let graph = GraphBuilder::default()
        .push_rx(-0.5, 0)
        .expect("Angle should be valid")
        .push_rz(15.0, 0)
        .expect("Angle should be valid")
        .build();

    let rule = canonicalization::normalize_angles();
    let matches = rule.detect(&graph);

    assert_eq!(matches.len(), 2);
    let positions: HashSet<_> = matches
        .iter()
        .flat_map(|position| position.iter())
        .copied()
        .collect();
    assert!(positions.contains(&Position::new(0, 0)));
    assert!(positions.contains(&Position::new(0, 1)));
}

#[test]
fn normalize_angles_close_to_boundary_in_range() {
    use crate::domain::math;
    let in_range_low = 1e-7;
    let in_range_high = math::FULL_CYCLE - 1e-7;

    let graph = GraphBuilder::default()
        .push_rx(in_range_low, 0)
        .expect("Angle should be valid")
        .push_rz(in_range_high, 0)
        .expect("Angle should be valid")
        .build();

    let rule = canonicalization::normalize_angles();
    let matches = rule.detect(&graph);

    assert!(matches.is_empty());
}

#[test]
fn normalize_angles_close_to_boundary_detected() {
    use crate::domain::math;
    let below_zero = -1e-7_f64;
    let above_full_cycle = math::FULL_CYCLE + 1e-7_f64;

    let graph = GraphBuilder::default()
        .push_rx(below_zero, 0)
        .expect("Angle should be valid")
        .push_rz(above_full_cycle, 0)
        .expect("Angle should be valid")
        .build();

    let rule = canonicalization::normalize_angles();
    let matches = rule.detect(&graph);

    assert_eq!(matches.len(), 2);
    let positions: HashSet<_> = matches
        .iter()
        .flat_map(|position| position.iter())
        .copied()
        .collect();
    assert!(positions.contains(&Position::new(0, 0)));
    assert!(positions.contains(&Position::new(0, 1)));
}

#[test]
fn normalize_angles_no_out_of_range() {
    let graph = GraphBuilder::default().push_x(0).push_h(0).build();

    let rule = canonicalization::normalize_angles();
    let matches = rule.detect(&graph);

    assert!(matches.is_empty());
}

#[test]
fn normalize_bits_detects_gap() {
    let graph = GraphBuilder::default()
        .push_measure(0, 0)
        .push_measure(0, 2)
        .build();

    let rule = canonicalization::normalize_bits();
    let matches = rule.detect(&graph);

    assert_eq!(matches.len(), 1);
    assert!(matches[0].contains(&Position::new(0, 1)));
}

#[test]
fn normalize_bits_no_gap() {
    let graph = GraphBuilder::default()
        .push_measure(0, 0)
        .push_measure(0, 1)
        .build();

    let rule = canonicalization::normalize_bits();
    let matches = rule.detect(&graph);

    assert!(matches.is_empty());
}

#[test]
fn normalize_bits_no_bit_zero() {
    let graph = GraphBuilder::default()
        .push_measure(0, 1)
        .push_measure(0, 2)
        .build();

    let rule = canonicalization::normalize_bits();
    let matches = rule.detect(&graph);

    assert!(matches.is_empty());
}

#[test]
fn normalize_bits_no_measurements() {
    let graph = GraphBuilder::default().push_x(0).build();

    let rule = canonicalization::normalize_bits();
    let matches = rule.detect(&graph);

    assert!(matches.is_empty());
}
