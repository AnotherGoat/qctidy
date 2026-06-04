use std::f64::consts::FRAC_PI_2;

use crate::domain::circuit::circuit_mother;

#[test]
fn empty_circuit() {
    let circuit = circuit_mother::empty();

    assert!(circuit.to_string().is_empty());
}

#[test]
fn single_id_gate() {
    let circuit = circuit_mother::single_id();
    let result = circuit.to_string();
    let expected = r"   ┌────┐
0: ┤ Id ├
   └────┘";

    assert_eq!(result, expected);
}

#[test]
fn single_h_gate() {
    let circuit = circuit_mother::single_h();
    let result = circuit.to_string();
    let expected = r"   ┌───┐
0: ┤ H ├
   └───┘";

    assert_eq!(result, expected);
}

#[test]
fn single_x_gate() {
    let circuit = circuit_mother::single_x();
    let result = circuit.to_string();
    let expected = r"   ┌───┐
0: ┤ X ├
   └───┘";

    assert_eq!(result, expected);
}

#[test]
fn single_y_gate() {
    let circuit = circuit_mother::single_y();
    let result = circuit.to_string();
    let expected = r"   ┌───┐
0: ┤ Y ├
   └───┘";

    assert_eq!(result, expected);
}

#[test]
fn single_z_gate() {
    let circuit = circuit_mother::single_z();
    let result = circuit.to_string();
    let expected = r"   ┌───┐
0: ┤ Z ├
   └───┘";

    assert_eq!(result, expected);
}

#[test]
fn rotation_p_gate() {
    let circuit = circuit_mother::single_p(FRAC_PI_2);
    let result = circuit.to_string();
    let expected = r"   ┌────────┐
0: ┤ P(π/2) ├
   └────────┘";

    assert_eq!(result, expected);
}

#[test]
fn rotation_rx_gate() {
    let circuit = circuit_mother::single_rx(FRAC_PI_2);
    let result = circuit.to_string();
    let expected = r"   ┌─────────┐
0: ┤ RX(π/2) ├
   └─────────┘";

    assert_eq!(result, expected);
}

#[test]
fn rotation_ry_gate() {
    let circuit = circuit_mother::single_ry(FRAC_PI_2);
    let result = circuit.to_string();
    let expected = r"   ┌─────────┐
0: ┤ RY(π/2) ├
   └─────────┘";

    assert_eq!(result, expected);
}

#[test]
fn rotation_rz_gate() {
    let circuit = circuit_mother::single_rz(FRAC_PI_2);
    let result = circuit.to_string();
    let expected = r"   ┌─────────┐
0: ┤ RZ(π/2) ├
   └─────────┘";

    assert_eq!(result, expected);
}

#[test]
fn single_s_gate() {
    let circuit = circuit_mother::single_s();
    let result = circuit.to_string();
    let expected = r"   ┌───┐
0: ┤ S ├
   └───┘";

    assert_eq!(result, expected);
}

#[test]
fn single_sdg_gate() {
    let circuit = circuit_mother::single_sdg();
    let result = circuit.to_string();
    let expected = r"   ┌────┐
0: ┤ S† ├
   └────┘";

    assert_eq!(result, expected);
}

#[test]
fn single_sx_gate() {
    let circuit = circuit_mother::single_sx();
    let result = circuit.to_string();
    let expected = r"   ┌────┐
0: ┤ √X ├
   └────┘";

    assert_eq!(result, expected);
}

#[test]
fn single_sy_gate() {
    let circuit = circuit_mother::single_sy();
    let result = circuit.to_string();
    let expected = r"   ┌────┐
0: ┤ √Y ├
   └────┘";

    assert_eq!(result, expected);
}

#[test]
fn single_t_gate() {
    let circuit = circuit_mother::single_t();
    let result = circuit.to_string();
    let expected = r"   ┌───┐
0: ┤ T ├
   └───┘";

    assert_eq!(result, expected);
}

#[test]
fn single_tdg_gate() {
    let circuit = circuit_mother::single_tdg();
    let result = circuit.to_string();
    let expected = r"   ┌────┐
0: ┤ T† ├
   └────┘";

    assert_eq!(result, expected);
}

#[test]
fn measurement_on_bit_5() {
    let circuit = circuit_mother::single_measure();
    let result = circuit.to_string();
    let expected = r"   ┌──────┐
0: ┤ M(5) ├
   └──────┘";

    assert_eq!(result, expected);
}

#[test]
fn swap_gate() {
    let circuit = circuit_mother::single_swap();
    let result = circuit.to_string();
    let expected = r"
0: ×
   │
1: ×
    ";

    assert_eq!(result, expected);
}

#[test]
fn swap_gate_inverted() {
    let circuit = circuit_mother::single_swap_inverted();
    let result = circuit.to_string();
    let expected = r"
0: ×
   │
1: ×
    ";

    assert_eq!(result, expected);
}

#[test]
fn ch_gate_bottom() {
    let circuit = circuit_mother::single_ch_bottom();
    let result = circuit.to_string();
    let expected = r"
0: ──■──
   ┌─┴─┐
1: ┤ H ├
   └───┘";

    assert_eq!(result, expected);
}

#[test]
fn ch_gate_top() {
    let circuit = circuit_mother::single_ch_top();
    let result = circuit.to_string();
    let expected = r"   ┌───┐
0: ┤ H ├
   └─┬─┘
1: ──■──
        ";

    assert_eq!(result, expected);
}

#[test]
fn cx_gate_bottom() {
    let circuit = circuit_mother::single_cx_bottom();
    let result = circuit.to_string();
    let expected = r"
0: ──■──
   ┌─┴─┐
1: ┤ X ├
   └───┘";

    assert_eq!(result, expected);
}

#[test]
fn cx_gate_top() {
    let circuit = circuit_mother::single_cx_top();
    let result = circuit.to_string();
    let expected = r"   ┌───┐
0: ┤ X ├
   └─┬─┘
1: ──■──
        ";

    assert_eq!(result, expected);
}

#[test]
fn cy_gate_bottom() {
    let circuit = circuit_mother::single_cy_bottom();
    let result = circuit.to_string();
    let expected = r"
0: ──■──
   ┌─┴─┐
1: ┤ Y ├
   └───┘";

    assert_eq!(result, expected);
}

#[test]
fn cy_gate_top() {
    let circuit = circuit_mother::single_cy_top();
    let result = circuit.to_string();
    let expected = r"   ┌───┐
0: ┤ Y ├
   └─┬─┘
1: ──■──
        ";

    assert_eq!(result, expected);
}

#[test]
fn cz_gate() {
    let circuit = circuit_mother::single_cz();
    let result = circuit.to_string();
    let expected = r"
0: ■
   │
1: ■
    ";

    assert_eq!(result, expected);
}

#[test]
fn cz_gate_inverted() {
    let circuit = circuit_mother::single_cz_inverted();
    let result = circuit.to_string();
    let expected = r"
0: ■
   │
1: ■
    ";

    assert_eq!(result, expected);
}

#[test]
fn cp_gate() {
    let circuit = circuit_mother::single_cp(FRAC_PI_2);
    let result = circuit.to_string();
    let expected = r"
0: ■(π/2)
      │
1: ■(π/2)
         ";

    assert_eq!(result, expected);
}

#[test]
fn cp_gate_inverted() {
    let circuit = circuit_mother::single_cp_inverted(FRAC_PI_2);
    let result = circuit.to_string();
    let expected = r"
0: ■(π/2)
      │
1: ■(π/2)
         ";

    assert_eq!(result, expected);
}

#[test]
fn cswap_gate_top() {
    let circuit = circuit_mother::single_cswap_top();
    let result = circuit.to_string();
    let expected = r"
0: ■
   │
1: ×
   │
2: ×
    ";

    assert_eq!(result, expected);
}

#[test]
fn cswap_gate_middle() {
    let circuit = circuit_mother::single_cswap_middle();
    let result = circuit.to_string();
    let expected = r"
0: ×
   │
1: ■
   │
2: ×
    ";

    assert_eq!(result, expected);
}

#[test]
fn cswap_gate_bottom() {
    let circuit = circuit_mother::single_cswap_bottom();
    let result = circuit.to_string();
    let expected = r"
0: ×
   │
1: ×
   │
2: ■
    ";

    assert_eq!(result, expected);
}

#[test]
fn ccx_gate_bottom() {
    let circuit = circuit_mother::single_ccx_bottom();
    let result = circuit.to_string();
    let expected = r"
0: ──■──
     │
1: ──■──
   ┌─┴─┐
2: ┤ X ├
   └───┘";

    assert_eq!(result, expected);
}

#[test]
fn ccx_gate_middle() {
    let circuit = circuit_mother::single_ccx_middle();
    let result = circuit.to_string();
    let expected = r"
0: ──■──
   ┌─┴─┐
1: ┤ X ├
   └─┬─┘
2: ──■──
        ";

    assert_eq!(result, expected);
}

#[test]
fn ccx_gate_top() {
    let circuit = circuit_mother::single_ccx_top();
    let result = circuit.to_string();
    let expected = r"   ┌───┐
0: ┤ X ├
   └─┬─┘
1: ──■──
     │
2: ──■──
        ";

    assert_eq!(result, expected);
}

#[test]
fn ccz_gate() {
    let circuit = circuit_mother::single_ccz();
    let result = circuit.to_string();
    let expected = r"
0: ■
   │
1: ■
   │
2: ■
    ";

    assert_eq!(result, expected);
}
