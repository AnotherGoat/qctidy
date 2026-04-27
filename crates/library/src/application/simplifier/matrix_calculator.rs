use std::{f64::consts::PI, sync::LazyLock};

use faer::{complex::Complex64, prelude::*};

use crate::{
    Graph,
    domain::math,
    dto::{GateOperation, mapper},
};

const EPSILON: f64 = math::ABSOLUTE_TOLERANCE;

const ONE: Complex64 = Complex64::new(1.0_f64, 0.0_f64);
const I: Complex64 = Complex64::new(0.0_f64, 1.0_f64);
static HALF_SQRT: LazyLock<f64> = LazyLock::new(|| 1.0_f64 / 2.0_f64.sqrt());

static ID_MATRIX: LazyLock<Mat<Complex64>> = LazyLock::new(|| identity(2));

#[expect(clippy::unnested_or_patterns)]
static H_MATRIX: LazyLock<Mat<Complex64>> = LazyLock::new(|| {
    Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) | (0, 1) | (1, 0) => *HALF_SQRT * ONE,
        (1, 1) => *HALF_SQRT * -ONE,
        _ => Complex64::default(),
    })
});

static X_MATRIX: LazyLock<Mat<Complex64>> = LazyLock::new(|| {
    Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 1) | (1, 0) => ONE,
        _ => Complex64::default(),
    })
});

static Y_MATRIX: LazyLock<Mat<Complex64>> = LazyLock::new(|| {
    Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 1) => -I,
        (1, 0) => I,
        _ => Complex64::default(),
    })
});

static Z_MATRIX: LazyLock<Mat<Complex64>> = LazyLock::new(|| {
    Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) => ONE,
        (1, 1) => -ONE,
        _ => Complex64::default(),
    })
});

static S_MATRIX: LazyLock<Mat<Complex64>> = LazyLock::new(|| {
    Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) => ONE,
        (1, 1) => I,
        _ => Complex64::default(),
    })
});

static SDG_MATRIX: LazyLock<Mat<Complex64>> = LazyLock::new(|| {
    Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) => ONE,
        (1, 1) => -I,
        _ => Complex64::default(),
    })
});

static SX_MATRIX: LazyLock<Mat<Complex64>> = LazyLock::new(|| {
    Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) | (1, 1) => 0.5_f64 * (ONE + I),
        (0, 1) | (1, 0) => 0.5_f64 * (ONE - I),
        _ => Complex64::default(),
    })
});

#[expect(clippy::unnested_or_patterns)]
static SY_MATRIX: LazyLock<Mat<Complex64>> = LazyLock::new(|| {
    Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) | (1, 0) | (1, 1) => 0.5_f64 * (ONE + I),
        (0, 1) => -0.5_f64 * (ONE + I),
        _ => Complex64::default(),
    })
});

static T_MATRIX: LazyLock<Mat<Complex64>> = LazyLock::new(|| {
    Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) => ONE,
        (1, 1) => (I * (PI / 4.0_f64)).exp(),
        _ => Complex64::default(),
    })
});

static TDG_MATRIX: LazyLock<Mat<Complex64>> = LazyLock::new(|| {
    Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) => ONE,
        (1, 1) => (-I * (PI / 4.0_f64)).exp(),
        _ => Complex64::default(),
    })
});

/// Check whether the unitary matrices of two quantum graphs are equivalent by normalizing global phase.
///
/// Uses `NumPy`'s default tolerance values, which are 1e-5 for relative and 1e-8 for absolute.
pub(crate) fn are_graphs_equivalent(first: &Graph, second: &Graph) -> bool {
    are_matrices_equivalent(&graph_circuit_matrix(first), &graph_circuit_matrix(second))
}

/// Check whether two unitary matrices are equivalent by normalizing global phase.
///
/// Uses `NumPy`'s default tolerance values, which are 1e-5 for relative and 1e-8 for absolute.
pub(crate) fn are_matrices_equivalent(first: &Mat<Complex64>, second: &Mat<Complex64>) -> bool {
    let phase = compute_global_phase(first, second);
    let adjusted = Scale(phase) * second;

    math::are_matrices_equal(first, &adjusted)
}

fn compute_global_phase(first: &Mat<Complex64>, second: &Mat<Complex64>) -> Complex64 {
    for row in 0..first.nrows() {
        for column in 0..first.ncols() {
            let divisor = second[(row, column)];

            if divisor.norm() > EPSILON {
                return first[(row, column)] / divisor;
            }
        }
    }

    ONE
}

/// Get the matrix representation of the circuit represented by the graph.
///
/// Measurement gates are completely ignored.
/// The output will always be a square matrix (rows == columns).
pub(crate) fn graph_circuit_matrix(graph: &Graph) -> Mat<Complex64> {
    let height = graph.height();
    let size = calculate_size(height);

    if graph.is_empty() {
        return identity(size);
    }

    let mut result = identity(size);
    let operations = mapper::graph_to_operations(graph);

    for operation in operations {
        if let Some(unitary) = operation_to_matrix(&operation, height) {
            result = &unitary * &result;
        }
    }

    result
}

const fn calculate_size(qubit_count: usize) -> usize {
    1 << qubit_count
}

fn identity(side: usize) -> Mat<Complex64> {
    Mat::identity(side, side)
}

fn zeros(side: usize) -> Mat<Complex64> {
    Mat::zeros(side, side)
}

fn operation_to_matrix(operation: &GateOperation, height: usize) -> Option<Mat<Complex64>> {
    use GateOperation::*;

    let matrix = match *operation {
        ID { .. } | Measure { .. } => return None,
        H { qubit } => embed_single_gate(height, qubit, &H_MATRIX),
        X { qubit } => embed_single_gate(height, qubit, &X_MATRIX),
        Y { qubit } => embed_single_gate(height, qubit, &Y_MATRIX),
        Z { qubit } => embed_single_gate(height, qubit, &Z_MATRIX),
        P { angle, qubit } => embed_single_gate(height, qubit, &phase_matrix(angle)),
        RX { angle, qubit } => embed_single_gate(height, qubit, &rx_matrix(angle)),
        RY { angle, qubit } => embed_single_gate(height, qubit, &ry_matrix(angle)),
        RZ { angle, qubit } => embed_single_gate(height, qubit, &rz_matrix(angle)),
        S { qubit } => embed_single_gate(height, qubit, &S_MATRIX),
        SDG { qubit } => embed_single_gate(height, qubit, &SDG_MATRIX),
        SX { qubit } => embed_single_gate(height, qubit, &SX_MATRIX),
        SY { qubit } => embed_single_gate(height, qubit, &SY_MATRIX),
        T { qubit } => embed_single_gate(height, qubit, &T_MATRIX),
        TDG { qubit } => embed_single_gate(height, qubit, &TDG_MATRIX),
        Swap { qubit1, qubit2 } => swap_gate(height, qubit1, qubit2),
        CH { control, target } => control_gate(height, &[control], target, &H_MATRIX),
        CX { control, target } => control_gate(height, &[control], target, &X_MATRIX),
        CY { control, target } => control_gate(height, &[control], target, &Y_MATRIX),
        CZ { qubit1, qubit2 } => control_gate(height, &[qubit1], qubit2, &Z_MATRIX),
        CP {
            angle,
            qubit1,
            qubit2,
        } => control_gate(height, &[qubit1], qubit2, &phase_matrix(angle)),
        CSwap {
            control,
            target1,
            target2,
        } => control_swap(height, control, target1, target2),
        CCX {
            control1,
            control2,
            target,
        } => control_gate(height, &[control1, control2], target, &X_MATRIX),
        CCZ {
            qubit1,
            qubit2,
            qubit3,
        } => control_gate(height, &[qubit1, qubit2], qubit3, &Z_MATRIX),
    };

    Some(matrix)
}

fn embed_single_gate(height: usize, qubit: usize, base: &Mat<Complex64>) -> Mat<Complex64> {
    let mut result = identity(1);

    for row in 0..height {
        if row == qubit {
            result = result.kron(base);
        } else {
            result = result.kron(&*ID_MATRIX);
        }
    }

    result
}

fn phase_matrix(angle: f64) -> Mat<Complex64> {
    Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) => ONE,
        (1, 1) => (I * angle).exp(),
        _ => Complex64::default(),
    })
}

fn rx_matrix(angle: f64) -> Mat<Complex64> {
    let cosine = (angle / 2.0).cos();
    let sine = (angle / 2.0).sin();

    Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) | (1, 1) => cosine * ONE,
        (0, 1) | (1, 0) => -sine * I,
        _ => Complex64::default(),
    })
}

fn ry_matrix(angle: f64) -> Mat<Complex64> {
    let cosine = (angle / 2.0).cos();
    let sine = (angle / 2.0).sin();

    Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) | (1, 1) => cosine * ONE,
        (0, 1) => -sine * ONE,
        (1, 0) => sine * ONE,
        _ => Complex64::default(),
    })
}

fn rz_matrix(angle: f64) -> Mat<Complex64> {
    let half_angle = angle / 2.0_f64;

    Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) => (I * -half_angle).exp(),
        (1, 1) => (I * half_angle).exp(),
        _ => Complex64::default(),
    })
}

fn control_gate(
    height: usize,
    controls: &[usize],
    target: usize,
    base: &Mat<Complex64>,
) -> Mat<Complex64> {
    let size = calculate_size(height);
    let mut unitary = zeros(size);

    for column in 0..size {
        let bits: Vec<usize> = (0..height).rev().map(|row| (column >> row) & 1).collect();

        if controls.iter().all(|&control| bits[control] == 1) {
            let target_bit = bits[target];

            for new_target in 0..2 {
                let mut new_bits = bits.clone();
                new_bits[target] = new_target;

                let row = new_bits
                    .iter()
                    .fold(0, |accumulator, &other| (accumulator << 1_i32) | other);

                unitary[(row, column)] = base[(new_target, target_bit)];
            }
        } else {
            unitary[(column, column)] = ONE;
        }
    }

    unitary
}

fn swap_gate(height: usize, qubit1: usize, qubit2: usize) -> Mat<Complex64> {
    let size = calculate_size(height);
    let mut unitary = zeros(size);

    for column in 0..size {
        let mut bits: Vec<usize> = (0..height).rev().map(|row| (column >> row) & 1).collect();

        bits.swap(qubit1, qubit2);

        let row = bits
            .iter()
            .fold(0, |accumulator, &other| (accumulator << 1_i32) | other);

        unitary[(row, column)] = ONE;
    }

    unitary
}

fn control_swap(height: usize, control: usize, target1: usize, target2: usize) -> Mat<Complex64> {
    let size = calculate_size(height);
    let mut unitary = zeros(size);

    for column in 0..size {
        let mut bits: Vec<usize> = (0..height).rev().map(|row| (column >> row) & 1).collect();

        if bits[control] == 1 {
            bits.swap(target1, target2);
        }

        let row = bits
            .iter()
            .fold(0, |accumulator, &other| (accumulator << 1_i32) | other);
        unitary[(row, column)] = ONE;
    }

    unitary
}
