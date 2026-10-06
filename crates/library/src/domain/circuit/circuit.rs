use getset::{CopyGetters, Getters};

use crate::{GateOperation, Position};

/// A collection of gate operations applied in the order of storage.
///
/// Stores the qubit count to differentiate circuits with different qubit counts but the same operations.
#[derive(Debug, Clone, Getters, CopyGetters)]
pub struct Circuit {
    /// The minimum number of qubits in the circuit built from these operations.
    ///
    /// If the circuit has more qubits than this, it will grow accordingly when converted to a graph.
    #[get_copy = "pub"]
    qubit_count: usize,
    /// The operations applied to the circuit, in order.
    #[get = "pub"]
    operations: Vec<GateOperation>,
}

impl Circuit {
    #[must_use]
    pub fn new(initial_qubit_count: usize, operations: Vec<GateOperation>) -> Self {
        let max_qubit = operations.iter().flat_map(GateOperation::qubits).max();
        let qubit_count = max_qubit.map_or(initial_qubit_count, |max| {
            (max + 1).max(initial_qubit_count)
        });

        Self {
            qubit_count,
            operations,
        }
    }

    #[must_use]
    pub fn from_operations(operations: Vec<GateOperation>) -> Self {
        Self::new(0, operations)
    }

    /// Return the positions occupied by each operation, in the same order as the operations.
    ///
    /// Multi-qubit operations occupy one position per qubit, all in the same time step.
    /// The positions match the graph built from this circuit with [`Graph::from`](crate::Graph).
    #[must_use]
    pub fn operation_positions(&self) -> Vec<Vec<Position>> {
        let mut last_column: Vec<Option<usize>> = vec![None; self.qubit_count];
        let mut positions = Vec::with_capacity(self.operations.len());

        for operation in &self.operations {
            let qubits = operation.qubits();
            let column = qubits
                .iter()
                .filter_map(|&row| last_column.get(row).copied().flatten())
                .max()
                .map_or(0, |last| last + 1);

            for &row in &qubits {
                if let Some(slot) = last_column.get_mut(row) {
                    *slot = Some(column);
                }
            }

            positions.push(
                qubits
                    .into_iter()
                    .map(|row| Position::new(row, column))
                    .collect(),
            );
        }

        positions
    }
}
