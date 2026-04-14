use std::fmt;

use getset::Getters;

use super::GraphNodeView;

/// A read-only view centered on a node, including all connected and neighboring nodes.
#[derive(Debug, Clone, PartialEq, Getters)]
#[get = "pub"]
pub struct NodeEdgeView {
    /// The node at the center of the view.
    origin: GraphNodeView,
    /// The node to the left.
    left: Option<GraphNodeView>,
    /// The node to the right.
    right: Option<GraphNodeView>,
    /// The node swapped with this one.
    swaps_with: Option<GraphNodeView>,
    /// Nodes targeted by this node.
    targets: Vec<GraphNodeView>,
    /// Nodes controlling this node.
    controlled_by: Vec<GraphNodeView>,
    /// Nodes that work together with this node.
    works_with: Vec<GraphNodeView>,
}

impl fmt::Display for NodeEdgeView {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut extra_data = Vec::new();

        if let Some(left) = &self.left {
            extra_data.push(format!("left={}", left));
        }

        if let Some(right) = &self.right {
            extra_data.push(format!("right={}", right));
        }

        if let Some(swaps_with) = &self.swaps_with {
            extra_data.push(format!("swaps_with={}", swaps_with));
        }

        if !self.targets.is_empty() {
            let targets: Vec<String> = self.targets.iter().map(|node| node.to_string()).collect();
            extra_data.push(format!("targets={:?}", targets));
        }

        if !self.controlled_by.is_empty() {
            let controllers: Vec<String> = self
                .controlled_by
                .iter()
                .map(|node| node.to_string())
                .collect();
            extra_data.push(format!("controlled_by={:?}", controllers));
        }

        if !self.works_with.is_empty() {
            let works_with: Vec<String> = self
                .works_with
                .iter()
                .map(|node| node.to_string())
                .collect();
            extra_data.push(format!("works_with={:?}", works_with));
        }

        if extra_data.is_empty() {
            return write!(formatter, "{}", self.origin);
        }

        write!(formatter, "{}({})", self.origin, extra_data.join(", "))
    }
}

impl NodeEdgeView {
    /// Create a new NodeEdgeView.
    pub fn new(
        origin: GraphNodeView,
        left: Option<GraphNodeView>,
        right: Option<GraphNodeView>,
        swaps_with: Option<GraphNodeView>,
        targets: Vec<GraphNodeView>,
        controlled_by: Vec<GraphNodeView>,
        works_with: Vec<GraphNodeView>,
    ) -> Self {
        Self {
            origin,
            left,
            right,
            swaps_with,
            targets,
            controlled_by,
            works_with,
        }
    }
}
