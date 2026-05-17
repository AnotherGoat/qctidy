use crate::{RuleRegistry, simplifier::rule::default};

#[test]
fn default_registry_is_valid() {
    let mut registry = RuleRegistry::new();
    default::register_all(&mut registry);
}
