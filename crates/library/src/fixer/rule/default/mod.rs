pub(crate) mod basis_change;
pub(crate) mod canonicalization;
pub(crate) mod control_reversal;
pub(crate) mod cx_reduction;
pub(crate) mod gate_synthesis;
pub(crate) mod miscellaneous;
pub(crate) mod pauli_propagation;
pub(crate) mod phase_compaction;
pub(crate) mod redundancy;

use crate::RuleRegistry;

pub(crate) fn register_all(registry: &mut RuleRegistry) {
    canonicalization::register(registry);
    redundancy::register(registry);
    phase_compaction::register(registry);
    basis_change::register(registry);
    control_reversal::register(registry);
    pauli_propagation::register(registry);
    gate_synthesis::register(registry);
    cx_reduction::register(registry);
    miscellaneous::register(registry);
}
