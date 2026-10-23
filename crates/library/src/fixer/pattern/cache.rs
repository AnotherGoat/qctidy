use crate::{GateType, Graph};

/// A fast cache for gate types present in qubits and time steps of a `Graph`.
///
/// Qubits correspond to matrix rows.
/// Time steps correspond to matrix columns.
#[derive(Debug, Clone)]
pub struct GraphCache {
    qubits: Vec<GateTypeBitset>,
    time_steps: Vec<GateTypeBitset>,
}

impl GraphCache {
    pub fn from_graph(graph: &Graph) -> Self {
        let mut qubits = vec![GateTypeBitset::new(); graph.qubit_count()];
        let mut time_steps = vec![GateTypeBitset::new(); graph.time_step_count()];

        for node in graph.iter_nodes() {
            let position = node.position();
            let gate_type = node.r#type();

            qubits[position.qubit()].insert(gate_type);
            time_steps[position.time_step()].insert(gate_type);
        }

        Self { qubits, time_steps }
    }

    pub fn qubits(&self) -> &[GateTypeBitset] {
        &self.qubits
    }

    pub fn time_steps(&self) -> &[GateTypeBitset] {
        &self.time_steps
    }

    pub fn qubit(&self, qubit: usize) -> Option<&GateTypeBitset> {
        self.qubits.get(qubit)
    }

    pub fn time_step(&self, time_step: usize) -> Option<&GateTypeBitset> {
        self.time_steps.get(time_step)
    }
}

/// A compact bitset of gate types.
///
/// Assumes that `GateType` has at most 64 variants.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateTypeBitset {
    bits: u64,
}

impl GateTypeBitset {
    /// Create an empty `GateTypeBitset`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a `GateType` to the bitset.
    pub const fn insert(&mut self, gate_type: GateType) {
        self.bits |= 1 << gate_type as u8;
    }

    /// Remove a `GateType` from the bitset.
    pub const fn remove(&mut self, gate_type: GateType) {
        self.bits &= !(1 << gate_type as u8);
    }

    /// Check whether the bitset contains a `GateType` or not.
    pub const fn contains(self, gate_type: GateType) -> bool {
        self.bits & (1 << gate_type as u8) != 0
    }

    pub const fn is_superset_of(self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }

    pub const fn intersects(self, other: Self) -> bool {
        (self.bits & other.bits) != 0
    }

    pub const fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    pub const fn intersection(self, other: Self) -> Self {
        Self {
            bits: self.bits & other.bits,
        }
    }

    pub const fn is_empty(self) -> bool {
        self.bits == 0
    }

    pub const fn clear(&mut self) {
        self.bits = 0;
    }

    pub const fn bits(self) -> u64 {
        self.bits
    }
}
