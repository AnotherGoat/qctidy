use std::fmt;

use crate::{
    AngleFormat, ContextualNodeView, EdgeView, NodeView,
    interface::display::angle_formatter::{self, PiFormat},
};

impl fmt::Display for NodeView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display(PiFormat::Lowercase))
    }
}

impl NodeView {
    /// Display a `NodeView` in a customizable format.
    ///
    /// Gives an output similar to `to_string`, but with more control over its format.
    #[must_use]
    pub fn display(&self, pi_format: PiFormat) -> String {
        let type_data = self.r#type().to_string().to_ascii_uppercase();

        let angle_data = self
            .angle()
            .map(|angle| {
                format!(
                    "(angle={})",
                    angle_formatter::format(angle, AngleFormat::Algebra, pi_format)
                )
            })
            .unwrap_or_default();

        let bit_data = self
            .bit()
            .map(|bit| format!("(bit={bit})"))
            .unwrap_or_default();

        format!(
            "{}{}{} at {}",
            type_data,
            angle_data,
            bit_data,
            self.position()
        )
    }
}

impl fmt::Display for EdgeView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display(PiFormat::Lowercase))
    }
}

impl EdgeView {
    /// Display an `EdgeView` in a customizable from -> to format.
    ///
    /// Gives an output similar to `to_string`, but with more control over its format.
    #[must_use]
    pub fn display(&self, pi_format: PiFormat) -> String {
        format!(
            "[{}] from {} to {}",
            self.r#type(),
            self.start().display(pi_format),
            self.end().display(pi_format),
        )
    }
}

impl fmt::Display for ContextualNodeView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display(PiFormat::Lowercase))
    }
}

impl ContextualNodeView {
    /// Display a `ContextualNodeView` in a customizable format.
    ///
    /// Gives an output similar to `to_string`, but with more control over its format.
    #[must_use]
    pub fn display(&self, pi_format: PiFormat) -> String {
        let origin_data = self.origin().display(pi_format);
        let mut extra_data = Vec::new();

        if let Some(left) = *self.left() {
            extra_data.push(format!("left={}", left.display(pi_format)));
        }

        if let Some(right) = *self.right() {
            extra_data.push(format!("right={}", right.display(pi_format)));
        }

        if !self.targets().is_empty() {
            let targets: Vec<String> = self
                .targets()
                .iter()
                .map(|target| target.display(pi_format))
                .collect();
            extra_data.push(format!("targets={targets:?}"));
        }

        if !self.controlled_by().is_empty() {
            let controllers: Vec<String> = self
                .controlled_by()
                .iter()
                .map(|controller| controller.display(pi_format))
                .collect();
            extra_data.push(format!("controlled_by={controllers:?}"));
        }

        if let Some(swaps_with) = *self.swaps_with() {
            extra_data.push(format!("swaps_with={}", swaps_with.display(pi_format)));
        }

        if !self.works_with().is_empty() {
            let works_with: Vec<String> = self
                .works_with()
                .iter()
                .map(|partner| partner.display(pi_format))
                .collect();
            extra_data.push(format!("works_with={works_with:?}"));
        }

        if extra_data.is_empty() {
            return origin_data;
        }

        format!("{}({})", origin_data, extra_data.join(", "))
    }
}
