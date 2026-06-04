use crate::{Circuit, GateOperation};

/// Create an empty circuit.
pub(crate) fn empty() -> Circuit {
    Circuit::from_operations(vec![])
}

/// Create a circuit with a single ID gate.
pub(crate) fn single_id() -> Circuit {
    Circuit::from_operations(vec![GateOperation::id(0)])
}

/// Create a circuit with a single H gate.
pub(crate) fn single_h() -> Circuit {
    Circuit::from_operations(vec![GateOperation::h(0)])
}

/// Create a circuit with a single X gate.
pub(crate) fn single_x() -> Circuit {
    Circuit::from_operations(vec![GateOperation::x(0)])
}

/// Create a circuit with a single Y gate.
pub(crate) fn single_y() -> Circuit {
    Circuit::from_operations(vec![GateOperation::y(0)])
}

/// Create a circuit with a single Z gate.
pub(crate) fn single_z() -> Circuit {
    Circuit::from_operations(vec![GateOperation::z(0)])
}

/// Create a circuit with a single P gate.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_p(angle: f64) -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_p(angle, 0).unwrap()])
}

/// Create a circuit with a single RX gate.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_rx(angle: f64) -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_rx(angle, 0).unwrap()])
}

/// Create a circuit with a single RY gate.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_ry(angle: f64) -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_ry(angle, 0).unwrap()])
}

/// Create a circuit with a single RZ gate.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_rz(angle: f64) -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_rz(angle, 0).unwrap()])
}

/// Create a circuit with a single S gate.
pub(crate) fn single_s() -> Circuit {
    Circuit::from_operations(vec![GateOperation::s(0)])
}

/// Create a circuit with a single SDG gate.
pub(crate) fn single_sdg() -> Circuit {
    Circuit::from_operations(vec![GateOperation::sdg(0)])
}

/// Create a circuit with a single SX gate.
pub(crate) fn single_sx() -> Circuit {
    Circuit::from_operations(vec![GateOperation::sx(0)])
}

/// Create a circuit with a single SY gate.
pub(crate) fn single_sy() -> Circuit {
    Circuit::from_operations(vec![GateOperation::sy(0)])
}

/// Create a circuit with a single T gate.
pub(crate) fn single_t() -> Circuit {
    Circuit::from_operations(vec![GateOperation::t(0)])
}

/// Create a circuit with a single TDG gate.
pub(crate) fn single_tdg() -> Circuit {
    Circuit::from_operations(vec![GateOperation::tdg(0)])
}

/// Create a circuit with a single measurement gate.
pub(crate) fn single_measure() -> Circuit {
    Circuit::from_operations(vec![GateOperation::measure(0, 5)])
}

/// Create a circuit with a single SWAP gate.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_swap() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_swap(0, 1).unwrap()])
}

/// Create a circuit with an inverted SWAP gate.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_swap_inverted() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_swap(1, 0).unwrap()])
}

/// Create a circuit with a CH gate with a target qubit on the bottom.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_ch_bottom() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_ch(0, 1).unwrap()])
}

/// Create a circuit with a CH gate with a target qubit on the top.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_ch_top() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_ch(1, 0).unwrap()])
}

/// Create a circuit with a CX gate with a target qubit on the bottom.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_cx_bottom() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_cx(0, 1).unwrap()])
}

/// Create a circuit with a CX gate with a target qubit on the top.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_cx_top() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_cx(1, 0).unwrap()])
}

/// Create a circuit with a CY gate with a target qubit on the bottom.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_cy_bottom() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_cy(0, 1).unwrap()])
}

/// Create a circuit with a CY gate with a target qubit on the top.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_cy_top() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_cy(1, 0).unwrap()])
}

/// Create a circuit with a CZ gate.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_cz() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_cz(0, 1).unwrap()])
}

/// Create a circuit with an inverted CZ gate.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_cz_inverted() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_cz(1, 0).unwrap()])
}

/// Create a circuit with a CP gate.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_cp(angle: f64) -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_cp(angle, 0, 1).unwrap()])
}

/// Create a circuit with an inverted CP gate.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_cp_inverted(angle: f64) -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_cp(angle, 1, 0).unwrap()])
}

/// Create a circuit with a CSwap gate with a control qubit on the bottom.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_cswap_bottom() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_c_swap(2, 0, 1).unwrap()])
}

/// Create a circuit with a CSwap gate with a control qubit on the middle.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_cswap_middle() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_c_swap(1, 0, 2).unwrap()])
}

/// Create a circuit with a CSwap gate with a control qubit on the top.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_cswap_top() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_c_swap(0, 1, 2).unwrap()])
}

/// Create a circuit with a CCX gate with a target qubit on the bottom.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_ccx_bottom() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_ccx(0, 1, 2).unwrap()])
}

/// Create a circuit with a CCX gate with a target qubit on the middle.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_ccx_middle() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_ccx(0, 2, 1).unwrap()])
}

/// Create a circuit with a CCX gate with a target qubit on the top.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_ccx_top() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_ccx(1, 2, 0).unwrap()])
}

/// Create a circuit with a CCZ gate.
#[expect(clippy::unwrap_used)]
pub(crate) fn single_ccz() -> Circuit {
    Circuit::from_operations(vec![GateOperation::try_ccz(0, 1, 2).unwrap()])
}
