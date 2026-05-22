use std::sync::Arc;

use function_name::named;

use crate::{GraphBuilder, PatternRule, RuleGroup, RuleMetadata, RuleRegistry};

pub(crate) fn register(registry: &mut RuleRegistry) {
    registry.register_all(vec![
        Arc::new(swap_synthesis()),
        Arc::new(ch_synthesis()),
        Arc::new(cy_synthesis()),
        Arc::new(cswap_synthesis()),
    ]);
}

#[named]
pub(crate) fn swap_synthesis() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_cx(1, 0)
        .expect("All qubits should be different")
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::default()
        .push_swap(0, 1)
        .expect("All qubits should be different")
        .build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Synthesize SWAP using its decomposition of 3 CX gates.",
            RuleGroup::GateSynthesis,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn ch_synthesis() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_s(1)
        .push_h(1)
        .push_t(1)
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_tdg(1)
        .push_h(1)
        .push_sdg(1)
        .build();

    let rhs = GraphBuilder::default()
        .push_ch(0, 1)
        .expect("All qubits should be different")
        .build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Synthesize CH using its decomposition of S, H, T, CX, Tdg, H and Sdg gates.",
            RuleGroup::GateSynthesis,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn cy_synthesis() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_sdg(1)
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_s(1)
        .build();

    let rhs = GraphBuilder::default()
        .push_cy(0, 1)
        .expect("All qubits should be different")
        .build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Synthesize CY using its decomposition of Sdg, CX and S gates.",
            RuleGroup::GateSynthesis,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn cswap_synthesis() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cx(2, 1)
        .expect("All qubits should be different")
        .push_ccx(0, 1, 2)
        .expect("All qubits should be different")
        .push_cx(2, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::default()
        .push_cswap(0, 1, 2)
        .expect("All qubits should be different")
        .build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Synthesize CSWAP using its decomposition of CX and CCX gates.",
            RuleGroup::GateSynthesis,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}
