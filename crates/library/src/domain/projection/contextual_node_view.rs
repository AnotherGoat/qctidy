use getset::Getters;
use newgen::New;

use crate::NodeView;

#[derive(Debug, Clone, PartialEq, Getters, New)]
#[get = "pub"]
#[new(pub, const)]
#[must_use]
pub struct ContextualNodeView {
    origin: NodeView,
    left: Option<NodeView>,
    right: Option<NodeView>,
    targets: Vec<NodeView>,
    controlled_by: Vec<NodeView>,
    swaps_with: Option<NodeView>,
    works_with: Vec<NodeView>,
}
