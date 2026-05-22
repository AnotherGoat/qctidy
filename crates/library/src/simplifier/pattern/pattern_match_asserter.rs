use std::collections::HashSet;

use inew::New;

use crate::Position;

use super::pattern_match::PatternMatch;

/// An assertion utility for verifying `PatternMatch` instances in tests.
///
/// Provides chainable assertion methods that panic on failure.
/// Follows the same pattern as `GraphAsserter`.
#[derive(New)]
#[new(pub(crate), const)]
pub(crate) struct PatternMatchAsserter<'a> {
    pattern_match: &'a PatternMatch,
}

impl PatternMatchAsserter<'_> {
    /// Assert that the match has the given rule ID.
    pub(crate) fn rule_id(&self, expected: &str) -> &Self {
        assert_eq!(*self.pattern_match.rule_id(), expected);
        self
    }

    /// Assert that the match covers the exact set of positions.
    pub(crate) fn covered_positions(&self, expected: &HashSet<Position>) -> &Self {
        let actual: HashSet<Position> = self
            .pattern_match
            .covered_positions()
            .iter()
            .copied()
            .collect();
        assert_eq!(actual, *expected);
        self
    }

    /// Assert that the match covers the given number of positions.
    pub(crate) fn covered_count(&self, expected: usize) -> &Self {
        assert_eq!(self.pattern_match.covered_positions().len(), expected);
        self
    }

    /// Assert that this match represents a redundant double-gate pattern on a single qubit such as H H or X X.
    pub(crate) fn single_qubit_double_gate(&self, expected_rule_id: &str) -> &Self {
        self.rule_id(expected_rule_id);
        let expected = HashSet::from([Position::new(0, 0), Position::new(0, 1)]);
        self.covered_positions(&expected)
    }

    /// Assert that this match represents a redundant double-gate pattern on two qubits, such as CX CX or SWAP SWAP.
    pub(crate) fn two_qubit_double_gate(&self, expected_rule_id: &str) -> &Self {
        self.rule_id(expected_rule_id);

        let expected = HashSet::from([
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(0, 1),
            Position::new(1, 1),
        ]);

        self.covered_positions(&expected)
    }

    /// Assert that this match represents a redundant double-gate pattern on three qubits, such as CCX CCX or CCZ CCZ.
    pub(crate) fn three_qubit_double_gate(&self, expected_rule_id: &str) -> &Self {
        self.rule_id(expected_rule_id);

        let expected = HashSet::from([
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(2, 0),
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(2, 1),
        ]);

        self.covered_positions(&expected)
    }
}
