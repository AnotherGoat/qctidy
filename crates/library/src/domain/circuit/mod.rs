pub(crate) mod circuit;
pub(crate) mod circuit_display;
pub(crate) mod conversions;
pub(crate) mod gate_operation;

pub use circuit::Circuit;
pub use gate_operation::{GateOperation, GateOperationError};

#[cfg(test)]
mod circuit_display_tests;
#[cfg(test)]
mod circuit_mother;
