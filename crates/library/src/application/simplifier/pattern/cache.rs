use crate::{GateType, Graph};

/// A fast cache for gate types present in rows and columns of a `Graph`.
///
/// Rows correspond to qubits.
/// Columns correspond to timesteps.
#[derive(Debug, Clone)]
pub struct GraphCache {
    rows: Vec<GateTypeBitset>,
    columns: Vec<GateTypeBitset>,
}

impl GraphCache {
    pub fn from_graph(graph: &Graph) -> Self {
        let mut rows = vec![GateTypeBitset::new(); graph.height()];
        let mut columns = vec![GateTypeBitset::new(); graph.width()];

        for node in graph.iter_nodes() {
            let position = node.position();
            let gate_type = node.r#type();

            rows[position.row()].insert(gate_type);
            columns[position.column()].insert(gate_type);
        }

        Self { rows, columns }
    }

    pub fn rows(&self) -> &[GateTypeBitset] {
        &self.rows
    }

    pub fn columns(&self) -> &[GateTypeBitset] {
        &self.columns
    }

    pub fn row(&self, row: usize) -> Option<&GateTypeBitset> {
        self.rows.get(row)
    }

    pub fn column(&self, column: usize) -> Option<&GateTypeBitset> {
        self.columns.get(column)
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
    pub fn insert(&mut self, gate_type: GateType) {
        self.bits |= 1 << u8::from(gate_type);
    }

    /// Remove a `GateType` from the bitset.
    pub fn remove(&mut self, gate_type: GateType) {
        self.bits &= !(1 << u8::from(gate_type));
    }

    /// Check whether the bitset contains a `GateType` or not.
    pub fn contains(&self, gate_type: GateType) -> bool {
        self.bits & (1 << u8::from(gate_type)) != 0
    }

    pub fn is_superset_of(&self, other: &Self) -> bool {
        (self.bits & other.bits) == other.bits
    }

    pub fn intersects(&self, other: &Self) -> bool {
        (self.bits & other.bits) != 0
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            bits: self.bits & other.bits,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn clear(&mut self) {
        self.bits = 0;
    }

    pub fn bits(&self) -> u64 {
        self.bits
    }
}
