use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, PI};

use crate::{ContextualNodeView, EdgeType, EdgeView, GateType, NodeView, PiFormat, Position};
use EdgeType::*;
use GateType::*;

#[test]
fn node_view_single_qubit_to_string() {
    let id_node = NodeView::new(ID, Position::new(0, 0), None, None);
    assert_eq!(id_node.to_string(), "ID at (0, 0)");

    let h_node = NodeView::new(H, Position::new(0, 1), None, None);
    assert_eq!(h_node.to_string(), "H at (0, 1)");

    let x_node = NodeView::new(X, Position::new(1, 0), None, None);
    assert_eq!(x_node.to_string(), "X at (1, 0)");

    let y_node = NodeView::new(Y, Position::new(1, 1), None, None);
    assert_eq!(y_node.to_string(), "Y at (1, 1)");

    let z_node = NodeView::new(Z, Position::new(2, 0), None, None);
    assert_eq!(z_node.to_string(), "Z at (2, 0)");

    let s_node = NodeView::new(S, Position::new(2, 1), None, None);
    assert_eq!(s_node.to_string(), "S at (2, 1)");

    let sdg_node = NodeView::new(SDG, Position::new(3, 0), None, None);
    assert_eq!(sdg_node.to_string(), "SDG at (3, 0)");

    let sx_node = NodeView::new(SX, Position::new(3, 1), None, None);
    assert_eq!(sx_node.to_string(), "SX at (3, 1)");

    let sy_node = NodeView::new(SY, Position::new(4, 0), None, None);
    assert_eq!(sy_node.to_string(), "SY at (4, 0)");

    let t_node = NodeView::new(T, Position::new(4, 1), None, None);
    assert_eq!(t_node.to_string(), "T at (4, 1)");

    let tdg_node = NodeView::new(TDG, Position::new(5, 0), None, None);
    assert_eq!(tdg_node.to_string(), "TDG at (5, 0)");
}

#[test]
fn node_view_display_with_angle() {
    let rx_node = NodeView::new(RX, Position::new(0, 0), Some(FRAC_PI_2), None);
    assert_eq!(rx_node.to_string(), "RX(angle=pi/2) at (0, 0)");

    let ry_node = NodeView::new(RY, Position::new(1, 2), Some(3.0_f64 * PI), None);
    assert_eq!(
        ry_node.display(PiFormat::Uppercase),
        "RY(angle=3PI) at (1, 2)"
    );

    let rz_node = NodeView::new(RZ, Position::new(0, 1), Some(2.0_f64 * FRAC_PI_3), None);
    assert_eq!(rz_node.display(PiFormat::Fancy), "RZ(angle=2π/3) at (0, 1)");

    let p_node = NodeView::new(P, Position::new(2, 0), Some(PI), None);
    assert_eq!(
        p_node.display(PiFormat::Custom { pi: "CustomPi" }),
        "P(angle=CustomPi) at (2, 0)"
    );
}

#[test]
fn node_view_rotation_with_zero_angle_to_string() {
    let node = NodeView::new(RZ, Position::new(1, 1), Some(0.0_f64), None);
    assert_eq!(node.to_string(), "RZ(angle=0) at (1, 1)");
}

#[test]
fn node_view_measurement_to_string() {
    let node = NodeView::new(Measure, Position::new(0, 0), None, Some(3));
    assert_eq!(node.to_string(), "M(bit=3) at (0, 0)");
}

#[test]
fn edge_view_right_to_string() {
    let start = NodeView::new(H, Position::new(0, 0), None, None);
    let end = NodeView::new(X, Position::new(0, 1), None, None);
    let edge = EdgeView::new(Right, start, end);

    let result = edge.display(PiFormat::Lowercase);

    assert_eq!(result, "[right] from H at (0, 0) to X at (0, 1)");
}

#[test]
fn edge_view_targets_to_string() {
    let start = NodeView::new(CX, Position::new(0, 0), None, None);
    let end = NodeView::new(CX, Position::new(1, 0), None, None);
    let edge = EdgeView::new(Targets, start, end);

    let result = edge.to_string();

    assert_eq!(result, "[targets] from CX at (0, 0) to CX at (1, 0)");
}

#[test]
fn edge_view_swaps_with_to_string() {
    let start = NodeView::new(Swap, Position::new(0, 0), None, None);
    let end = NodeView::new(Swap, Position::new(1, 0), None, None);
    let edge = EdgeView::new(SwapsWith, start, end);

    let result = edge.to_string();

    assert_eq!(result, "[swaps_with] from SWAP at (0, 0) to SWAP at (1, 0)");
}

#[test]
fn edge_view_works_with_to_string() {
    let start = NodeView::new(CCX, Position::new(0, 0), None, None);
    let end = NodeView::new(CCX, Position::new(1, 0), None, None);
    let edge = EdgeView::new(WorksWith, start, end);

    let result = edge.to_string();

    assert_eq!(result, "[works_with] from CCX at (0, 0) to CCX at (1, 0)");
}

#[test]
fn edge_view_display_with_angles() {
    let start = NodeView::new(RX, Position::new(0, 0), Some(FRAC_PI_2), None);
    let end = NodeView::new(RX, Position::new(0, 1), Some(FRAC_PI_3), None);
    let edge = EdgeView::new(Right, start, end);

    assert_eq!(
        edge.to_string(),
        "[right] from RX(angle=pi/2) at (0, 0) to RX(angle=pi/3) at (0, 1)"
    );
    assert_eq!(
        edge.display(PiFormat::Uppercase),
        "[right] from RX(angle=PI/2) at (0, 0) to RX(angle=PI/3) at (0, 1)"
    );
    assert_eq!(
        edge.display(PiFormat::Fancy),
        "[right] from RX(angle=π/2) at (0, 0) to RX(angle=π/3) at (0, 1)"
    );
    assert_eq!(
        edge.display(PiFormat::Custom { pi: "CustomPi" }),
        "[right] from RX(angle=CustomPi/2) at (0, 0) to RX(angle=CustomPi/3) at (0, 1)"
    );
}

#[test]
fn edge_view_display_with_bits() {
    let start = NodeView::new(Measure, Position::new(0, 0), None, Some(5));
    let end = NodeView::new(Measure, Position::new(0, 1), None, Some(0));
    let edge = EdgeView::new(Right, start, end);

    let result = edge.to_string();

    assert_eq!(
        result,
        "[right] from M(bit=5) at (0, 0) to M(bit=0) at (0, 1)"
    );
}

#[test]
fn contextual_node_view_to_string_origin_only() {
    let origin = NodeView::new(H, Position::new(0, 0), None, None);
    let view = ContextualNodeView::new(origin, None, None, vec![], vec![], None, vec![]);

    let result = view.to_string();

    assert_eq!(result, "H at (0, 0)");
}

#[test]
fn contextual_node_view_to_string_with_left() {
    let origin = NodeView::new(X, Position::new(0, 1), None, None);
    let left = NodeView::new(H, Position::new(0, 0), None, None);
    let view = ContextualNodeView::new(origin, Some(left), None, vec![], vec![], None, vec![]);

    let result = view.to_string();

    assert_eq!(result, "X at (0, 1) {left=H at (0, 0)}");
}

#[test]
fn contextual_node_view_to_string_with_right() {
    let origin = NodeView::new(H, Position::new(0, 0), None, None);
    let right = NodeView::new(X, Position::new(0, 1), None, None);
    let view = ContextualNodeView::new(origin, None, Some(right), vec![], vec![], None, vec![]);

    let result = view.to_string();

    assert_eq!(result, "H at (0, 0) {right=X at (0, 1)}");
}

#[test]
fn contextual_node_view_to_string_with_left_and_right() {
    let origin = NodeView::new(Y, Position::new(1, 1), None, None);
    let left = NodeView::new(X, Position::new(1, 0), None, None);
    let right = NodeView::new(Z, Position::new(1, 2), None, None);
    let view = ContextualNodeView::new(
        origin,
        Some(left),
        Some(right),
        vec![],
        vec![],
        None,
        vec![],
    );

    let result = view.to_string();

    assert_eq!(result, "Y at (1, 1) {left=X at (1, 0), right=Z at (1, 2)}");
}

#[test]
fn contextual_node_view_to_string_with_targets() {
    let origin = NodeView::new(CX, Position::new(0, 0), None, None);
    let target = NodeView::new(CX, Position::new(1, 0), None, None);
    let view = ContextualNodeView::new(origin, None, None, vec![target], vec![], None, vec![]);

    let result = view.to_string();

    assert_eq!(result, "CX at (0, 0) {targets=[\"CX at (1, 0)\"]}");
}

#[test]
fn contextual_node_view_to_string_with_multiple_targets() {
    let origin = NodeView::new(CCX, Position::new(0, 0), None, None);
    let target1 = NodeView::new(CCX, Position::new(1, 0), None, None);
    let target2 = NodeView::new(CCX, Position::new(2, 0), None, None);
    let view = ContextualNodeView::new(
        origin,
        None,
        None,
        vec![target1, target2],
        vec![],
        None,
        vec![],
    );

    let result = view.to_string();

    assert_eq!(
        result,
        "CCX at (0, 0) {targets=[\"CCX at (1, 0)\", \"CCX at (2, 0)\"]}"
    );
}

#[test]
fn contextual_node_view_to_string_with_controlled_by() {
    let origin = NodeView::new(CX, Position::new(1, 0), None, None);
    let controller = NodeView::new(CX, Position::new(0, 0), None, None);
    let view = ContextualNodeView::new(origin, None, None, vec![], vec![controller], None, vec![]);

    let result = view.to_string();

    assert_eq!(result, "CX at (1, 0) {controlled_by=[\"CX at (0, 0)\"]}");
}

#[test]
fn contextual_node_view_to_string_with_swaps_with() {
    let origin = NodeView::new(Swap, Position::new(0, 0), None, None);
    let partner = NodeView::new(Swap, Position::new(1, 0), None, None);
    let view = ContextualNodeView::new(origin, None, None, vec![], vec![], Some(partner), vec![]);

    let result = view.to_string();

    assert_eq!(result, "SWAP at (0, 0) {swaps_with=SWAP at (1, 0)}");
}

#[test]
fn contextual_node_view_to_string_with_works_with() {
    let origin = NodeView::new(CCX, Position::new(0, 0), None, None);
    let partner1 = NodeView::new(CCX, Position::new(1, 0), None, None);
    let partner2 = NodeView::new(CCX, Position::new(2, 0), None, None);
    let view = ContextualNodeView::new(
        origin,
        None,
        None,
        vec![],
        vec![],
        None,
        vec![partner1, partner2],
    );

    let result = view.to_string();

    assert_eq!(
        result,
        "CCX at (0, 0) {works_with=[\"CCX at (1, 0)\", \"CCX at (2, 0)\"]}"
    );
}

#[test]
fn contextual_node_view_to_string_with_all_relationships() {
    let origin = NodeView::new(CCX, Position::new(1, 1), None, None);
    let left = NodeView::new(ID, Position::new(1, 0), None, None);
    let right = NodeView::new(ID, Position::new(1, 2), None, None);
    let target = NodeView::new(CCX, Position::new(0, 1), None, None);
    let controller = NodeView::new(CCX, Position::new(2, 1), None, None);
    let partner = NodeView::new(CCX, Position::new(0, 1), None, None);

    let view = ContextualNodeView::new(
        origin,
        Some(left),
        Some(right),
        vec![target],
        vec![controller],
        None,
        vec![partner],
    );

    let result = view.to_string();

    assert_eq!(
        result,
        "CCX at (1, 1) {left=ID at (1, 0), right=ID at (1, 2), targets=[\"CCX at (0, 1)\"], controlled_by=[\"CCX at (2, 1)\"], works_with=[\"CCX at (0, 1)\"]}"
    );
}

#[test]
fn contextual_node_view_display_with_pi_format() {
    let origin = NodeView::new(P, Position::new(0, 0), Some(FRAC_PI_3), None);
    let view = ContextualNodeView::new(origin, None, None, vec![], vec![], None, vec![]);

    assert_eq!(view.display(PiFormat::Lowercase), "P(angle=pi/3) at (0, 0)");
    assert_eq!(view.display(PiFormat::Uppercase), "P(angle=PI/3) at (0, 0)");
    assert_eq!(view.display(PiFormat::Fancy), "P(angle=π/3) at (0, 0)");
    assert_eq!(
        view.display(PiFormat::Custom { pi: "Pi" }),
        "P(angle=Pi/3) at (0, 0)"
    );
}
