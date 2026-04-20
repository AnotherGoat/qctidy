use getset::Getters;
use inew::New;

use crate::NodeView;

/// A read-only projection centered on a node, including all connected and neighboring nodes.
#[derive(Debug, Clone, PartialEq, Getters, New)]
#[get = "pub"]
#[new(pub, const)]
pub struct ContextualNodeView {
    /// The node at the center of the view.
    origin: NodeView,
    /// The node to the left.
    left: Option<NodeView>,
    /// The node to the right.
    right: Option<NodeView>,
    /// Nodes targeted by this node.
    targets: Vec<NodeView>,
    /// Nodes controlling this node.
    controlled_by: Vec<NodeView>,
    /// The node swapped with this one.
    swaps_with: Option<NodeView>,
    /// Nodes that work together with this node.
    works_with: Vec<NodeView>,
}
