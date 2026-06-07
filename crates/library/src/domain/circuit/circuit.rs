use getset::{CopyGetters, Getters};

use crate::GateOperation;

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
}
