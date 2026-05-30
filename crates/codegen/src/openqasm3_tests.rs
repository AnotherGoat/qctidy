use qsimplify::GraphBuilder;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, PI};

use crate::openqasm3;

#[test]
#[expect(clippy::unwrap_used)]
fn generate_empty_circuit() {
    let graph = GraphBuilder::default().build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_single_qubit_circuit() {
    let graph = GraphBuilder::default()
        .push_h(0)
        .push_x(0)
        .push_y(0)
        .push_z(0)
        .push_t(0)
        .build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

qubit[1] q;
h q[0];
x q[0];
y q[0];
z q[0];
t q[0];";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_identity() {
    let graph = GraphBuilder::default().push_id(0).build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

qubit[1] q;
id q[0];";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_measurements() {
    let graph = GraphBuilder::default().push_h(0).push_measure(0, 0).build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

qubit[1] q;
bit[1] c;
h q[0];
measure q[0] -> c[0];";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_rotations() {
    let graph = GraphBuilder::default()
        .push_rx(PI, 0)
        .unwrap()
        .push_ry(FRAC_PI_2, 0)
        .unwrap()
        .push_rz(2.0 * FRAC_PI_3, 0)
        .unwrap()
        .push_p(5.0 * FRAC_PI_4, 0)
        .unwrap()
        .build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

qubit[1] q;
rx(pi) q[0];
ry(pi / 2) q[0];
rz(2 * pi / 3) q[0];
p(5 * pi / 4) q[0];";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_zero_rotations() {
    let graph = GraphBuilder::default()
        .push_p(0.0, 0)
        .unwrap()
        .push_p(1e-8, 0)
        .unwrap()
        .build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

qubit[1] q;
p(0) q[0];
p(0) q[0];";

    assert_eq!(actual, expected,);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_uncommon_rotations() {
    let graph = GraphBuilder::default()
        .push_rx(0.222, 0)
        .unwrap()
        .push_ry(1.333, 0)
        .unwrap()
        .push_rz(2.444, 0)
        .unwrap()
        .push_p(3.555, 0)
        .unwrap()
        .build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

qubit[1] q;
rx(0.222) q[0];
ry(1.333) q[0];
rz(2.444) q[0];
p(3.555) q[0];";

    assert_eq!(actual, expected);
}

#[test]
fn generate_circuit_with_sy_gate() {
    let graph = GraphBuilder::default().push_sy(0).build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

gate sy q {
    ry(-pi / 2) q;
}

qubit[1] q;
sy q[0];";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_two_qubits() {
    let graph = GraphBuilder::default()
        .push_cx(0, 1)
        .unwrap()
        .push_cy(1, 0)
        .unwrap()
        .build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

qubit[2] q;
cx q[0], q[1];
cy q[1], q[0];";

    assert_eq!(actual, expected);
}

#[test]
fn generate_circuit_orders_qubits() {
    let graph = GraphBuilder::default()
        .push_h(4)
        .push_x(2)
        .push_y(0)
        .push_z(1)
        .push_s(3)
        .build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

qubit[5] q;
y q[0];
z q[1];
x q[2];
s q[3];
h q[4];";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_orders_symmetrical_gates() {
    let graph = GraphBuilder::default()
        .push_swap(1, 0)
        .unwrap()
        .push_cz(2, 0)
        .unwrap()
        .push_swap(3, 1)
        .unwrap()
        .push_cz(3, 2)
        .unwrap()
        .build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

qubit[4] q;
swap q[0], q[1];
cz q[0], q[2];
swap q[1], q[3];
cz q[2], q[3];";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_three_qubits() {
    let graph = GraphBuilder::default()
        .push_ccx(0, 2, 1)
        .unwrap()
        .push_cswap(1, 0, 2)
        .unwrap()
        .push_ccz(0, 1, 2)
        .unwrap()
        .build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

qubit[3] q;
ccx q[0], q[2], q[1];
cswap q[1], q[0], q[2];
ccz q[0], q[1], q[2];";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_multiple_bits() {
    let graph = GraphBuilder::default()
        .push_h(0)
        .push_x(1)
        .push_y(2)
        .push_measure(0, 0)
        .push_measure(1, 1)
        .push_measure(2, 2)
        .build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

qubit[3] q;
bit[3] c;
h q[0];
x q[1];
y q[2];
measure q[0] -> c[0];
measure q[1] -> c[1];
measure q[2] -> c[2];";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_flipped_measurements() {
    let graph = GraphBuilder::default()
        .push_h(0)
        .push_z(1)
        .push_t(2)
        .push_measure(0, 1)
        .push_measure(1, 2)
        .push_measure(2, 0)
        .build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

qubit[3] q;
bit[3] c;
h q[0];
z q[1];
t q[2];
measure q[0] -> c[1];
measure q[1] -> c[2];
measure q[2] -> c[0];";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_cp_gate() {
    let graph = GraphBuilder::default()
        .push_cp(FRAC_PI_3, 0, 1)
        .unwrap()
        .build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

qubit[2] q;
cp(pi / 3) q[0], q[1];";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_ch_gate() {
    let graph = GraphBuilder::default().push_ch(0, 1).unwrap().build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

qubit[2] q;
ch q[0], q[1];";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_s_gates() {
    let graph = GraphBuilder::default()
        .push_s(0)
        .push_sdg(0)
        .push_sx(0)
        .push_t(0)
        .push_tdg(0)
        .build();
    let actual = openqasm3::generate(&graph).unwrap();

    let expected = "\
OPENQASM 3.0;
include \"stdgates.inc\";

qubit[1] q;
s q[0];
sdg q[0];
sx q[0];
t q[0];
tdg q[0];";

    assert_eq!(actual, expected);
}
