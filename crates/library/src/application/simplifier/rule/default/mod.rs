pub(crate) mod miscellaneous;
pub(crate) mod normalization;
pub(crate) mod redundancy;

use crate::RuleRegistry;

pub(crate) fn register_all(registry: &mut RuleRegistry) {
    redundancy::register(registry);
    normalization::register(registry);
    miscellaneous::register(registry);
}
