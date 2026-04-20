use getset::{CopyGetters, Getters};
use inew::New;

use crate::{EdgeType, NodeView};

/// A read-only projection of an edge in a `Graph`, which connects two nodes in a single direction.
#[derive(Debug, Clone, Copy, PartialEq, Getters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct EdgeView {
    /// The type of edge.
    #[get_copy = "pub"]
    r#type: EdgeType,
    /// Start node.
    #[get = "pub"]
    start: NodeView,
    /// End node.
    #[get = "pub"]
    end: NodeView,
}
