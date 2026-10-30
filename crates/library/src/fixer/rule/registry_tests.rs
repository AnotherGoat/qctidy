use crate::{RuleRegistry, fixer::rule::default};

#[test]
fn default_registry_is_valid() {
    let mut registry = RuleRegistry::new();
    default::register_all(&mut registry);
}

#[test]
fn default_rule_descriptions_do_not_end_with_a_period() {
    let mut registry = RuleRegistry::new();
    default::register_all(&mut registry);

    for rule in registry.iter() {
        let metadata = rule.metadata();

        assert!(
            !metadata.description().ends_with('.'),
            "{} should not end with a period: {}",
            metadata.code(),
            metadata.description()
        );
    }
}
