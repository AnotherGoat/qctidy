use std::collections::HashMap;
use std::fmt;

use getset::{CopyGetters, Getters};
use newgen::New;

use crate::RuleMetadata;

/// Configuration for graph fix rules.
#[derive(Debug, Clone, Getters, CopyGetters, New)]
#[new(pub)]
pub struct RuleConfiguration {
    /// The default severity for all the rules.
    #[get_copy = "pub"]
    default: RuleSeverity,
    /// Overrides for specific rules or categories, keyed by their code.
    #[get = "pub"]
    #[new(default)]
    overrides: HashMap<String, RuleSeverity>,
}

impl RuleConfiguration {
    /// Select a rule or a whole category by its code, enabling it as a warning.
    ///
    /// The selector is a rule code such as `R001`, or a category code such as `R`.
    /// A rule-specific override wins over a category override.
    pub fn select(&mut self, selector: &str) {
        self.overrides
            .insert(selector.to_owned(), RuleSeverity::Warn);
    }

    /// Ignore a rule or a whole category by its code.
    pub fn ignore(&mut self, selector: &str) {
        self.overrides
            .insert(selector.to_owned(), RuleSeverity::Off);
    }

    /// Get the severity for a rule.
    ///
    /// A rule-specific override wins over a category override.
    #[must_use]
    pub fn severity(&self, metadata: &RuleMetadata) -> RuleSeverity {
        let code: &str = metadata.code();
        let category: &str = metadata.category().code();

        self.overrides
            .get(code)
            .or_else(|| self.overrides.get(category))
            .copied()
            .unwrap_or(self.default)
    }
}

/// Severity reported for a graph fix rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleSeverity {
    /// The rule is skipped.
    Off,
    /// The rule is reported as a warning.
    Warn,
    /// The rule is reported as an error.
    Error,
}

impl fmt::Display for RuleSeverity {
    /// Obtain the name of this severity as a lowercase string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::Off => "off",
            Self::Warn => "warning",
            Self::Error => "error",
        };

        write!(f, "{name}")
    }
}
