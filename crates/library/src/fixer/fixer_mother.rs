use crate::{
    RuleConfiguration, RuleSeverity,
    fixer::{Fixer, rule::registry::DEFAULT_RULE_REGISTRY},
};

/// Create a `Fixer` with all the default rules enabled.
pub(crate) fn default() -> Fixer {
    Fixer::new(
        &DEFAULT_RULE_REGISTRY,
        vec![],
        &RuleConfiguration::new(RuleSeverity::Warn),
    )
}
