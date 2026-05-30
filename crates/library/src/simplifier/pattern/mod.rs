pub(crate) mod anchor;
pub(crate) mod cache;
pub(crate) mod matcher;
mod occupancy;
pub(crate) mod pattern_match;
pub(crate) mod replacer;

#[cfg(test)]
pub(crate) mod pattern_match_asserter;
#[cfg(test)]
mod pattern_match_tests;
#[cfg(test)]
mod replacer_tests;
