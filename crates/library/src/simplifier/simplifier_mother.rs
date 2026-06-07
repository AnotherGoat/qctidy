use crate::{
    RuleConfiguration, RuleLevel,
    simplifier::{Simplifier, rule::registry::DEFAULT_RULE_REGISTRY},
};

/// Create a `Simplifier` with all the default rules enabled.
pub(crate) fn default() -> Simplifier {
    Simplifier::new(
        &DEFAULT_RULE_REGISTRY,
        vec![],
        &RuleConfiguration::new(RuleLevel::Apply),
    )
}
