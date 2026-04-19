#![warn(missing_docs)]

/// Contains domain structures used to represent a `Graph`.
pub mod domain;
pub mod dto;
pub mod simplifier;
/// Contains general logic and text display utilities.
pub(crate) mod utils;
/// Provides views for nodes and edges inside a `Graph`.
pub mod view;
