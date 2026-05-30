use crate::{PatternRule, SimplificationRule, simplifier::rule::registry::DEFAULT_RULE_REGISTRY};

#[test]
fn replace_default_pattern_rules() {
    let registry = &DEFAULT_RULE_REGISTRY;

    for rule in registry.iter() {
        if let Some(pattern_rule) = rule.as_any().downcast_ref::<PatternRule>() {
            let mut lhs = pattern_rule.lhs().clone();
            let rhs = pattern_rule.rhs();

            let changed = pattern_rule.apply(&mut lhs);
            assert!(changed);
            assert_eq!(lhs, *rhs);
        }
    }
}
