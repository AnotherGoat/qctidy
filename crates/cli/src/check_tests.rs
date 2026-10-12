use qctidy::{RuleCategory, RuleMetadata, RuleSeverity, fixer};

use crate::check::build_configuration;
use crate::error::CliError;

fn codes(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn rule_in(category: RuleCategory) -> RuleMetadata {
    *fixer::default_rules()
        .iter()
        .find(|metadata| metadata.category() == category)
        .expect("the category should have at least one rule")
}

fn rule_outside(category: RuleCategory) -> RuleMetadata {
    *fixer::default_rules()
        .iter()
        .find(|metadata| metadata.category() != category)
        .expect("there should be rules outside the category")
}

#[test]
fn no_selectors_enables_every_rule() {
    let configuration = build_configuration(&[], &[]).unwrap();
    let redundancy = rule_in(RuleCategory::Redundancy);

    assert_eq!(configuration.severity(&redundancy), RuleSeverity::Warn);
}

#[test]
fn select_restricts_the_enabled_rules() {
    let configuration = build_configuration(&codes(&["R"]), &[]).unwrap();

    assert_eq!(
        configuration.severity(&rule_in(RuleCategory::Redundancy)),
        RuleSeverity::Warn
    );
    assert_eq!(
        configuration.severity(&rule_outside(RuleCategory::Redundancy)),
        RuleSeverity::Off
    );
}

#[test]
fn ignore_disables_the_given_rules() {
    let configuration = build_configuration(&[], &codes(&["R"])).unwrap();

    assert_eq!(
        configuration.severity(&rule_in(RuleCategory::Redundancy)),
        RuleSeverity::Off
    );
    assert_eq!(
        configuration.severity(&rule_outside(RuleCategory::Redundancy)),
        RuleSeverity::Warn
    );
}

#[test]
fn ignore_wins_over_select_for_the_same_selector() {
    let configuration = build_configuration(&codes(&["R"]), &codes(&["R"])).unwrap();

    assert_eq!(
        configuration.severity(&rule_in(RuleCategory::Redundancy)),
        RuleSeverity::Off
    );
}

#[test]
fn a_rule_selector_wins_over_a_category_selector() {
    let redundancy = rule_in(RuleCategory::Redundancy);
    let configuration = build_configuration(&codes(&["R"]), &codes(&[*redundancy.code()])).unwrap();

    assert_eq!(configuration.severity(&redundancy), RuleSeverity::Off);
}

#[test]
fn repeated_selectors_are_ignored() {
    let configuration = build_configuration(&codes(&["R", "R", "R"]), &[]).unwrap();

    assert_eq!(
        configuration.severity(&rule_in(RuleCategory::Redundancy)),
        RuleSeverity::Warn
    );
}

#[test]
fn unknown_selectors_are_rejected() {
    let error = build_configuration(&codes(&["NOPE"]), &[]).unwrap_err();

    assert!(matches!(
        error,
        CliError::UnknownRuleSelector { selector } if selector == "NOPE"
    ));
}
