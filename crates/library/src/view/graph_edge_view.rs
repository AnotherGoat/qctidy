use std::fmt;

use getset::{CopyGetters, Getters};

use crate::domain::EdgeType;

use super::GraphNodeView;

/// A read-only view of an edge in a `Graph`, which connects two nodes in a single direction.
#[derive(Debug, Clone, PartialEq, Getters, CopyGetters)]
pub struct GraphEdgeView {
    /// The type of edge.
    #[get_copy = "pub"]
    r#type: EdgeType,
    /// Start node.
    #[get = "pub"]
    start: GraphNodeView,
    /// End node.
    #[get = "pub"]
    end: GraphNodeView,
}

impl fmt::Display for GraphEdgeView {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[{}] from {} to {}",
            self.r#type, self.start, self.end
        )
    }
}

impl GraphEdgeView {
    /// Create a new `GraphEdgeView`.
    pub fn new(r#type: EdgeType, start: GraphNodeView, end: GraphNodeView) -> Self {
        Self { r#type, start, end }
    }
}
