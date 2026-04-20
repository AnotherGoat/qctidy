use std::fmt;

use crate::{
    ContextualNodeView, EdgeView, NodeView,
    interface::display::angle_formatter::{self, PiFormat},
};

impl fmt::Display for NodeView {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_data = self.r#type().to_string().to_ascii_uppercase();

        let angle_data = self
            .angle()
            .map(|angle| {
                format!(
                    "(angle={})",
                    angle_formatter::format(angle, PiFormat::Lowercase)
                )
            })
            .unwrap_or_default();

        let bit_data = self
            .bit()
            .map(|bit| format!("(bit={})", bit))
            .unwrap_or_default();

        write!(
            formatter,
            "{}{}{} at {}",
            type_data,
            angle_data,
            bit_data,
            self.position()
        )
    }
}

impl fmt::Display for EdgeView {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[{}] from {} to {}",
            self.r#type(),
            self.start(),
            self.end()
        )
    }
}

impl fmt::Display for ContextualNodeView {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut extra_data = Vec::new();

        if let Some(left) = &self.left() {
            extra_data.push(format!("left={}", left));
        }

        if let Some(right) = &self.right() {
            extra_data.push(format!("right={}", right));
        }

        if !self.targets().is_empty() {
            let targets: Vec<String> = self.targets().iter().map(|node| node.to_string()).collect();
            extra_data.push(format!("targets={:?}", targets));
        }

        if !self.controlled_by().is_empty() {
            let controllers: Vec<String> = self
                .controlled_by()
                .iter()
                .map(|node| node.to_string())
                .collect();
            extra_data.push(format!("controlled_by={:?}", controllers));
        }

        if let Some(swaps_with) = &self.swaps_with() {
            extra_data.push(format!("swaps_with={}", swaps_with));
        }

        if !self.works_with().is_empty() {
            let works_with: Vec<String> = self
                .works_with()
                .iter()
                .map(|node| node.to_string())
                .collect();
            extra_data.push(format!("works_with={:?}", works_with));
        }

        if extra_data.is_empty() {
            return write!(formatter, "{}", self.origin());
        }

        write!(formatter, "{}({})", self.origin(), extra_data.join(", "))
    }
}
