use std::f64::consts::{FRAC_PI_2, PI};
use std::sync::Arc;

use qctidy::{Circuit, GateOperation};
use qctidy_facade::{FixContext, Session};
use qctidy_ports::SourceLocation;

use crate::{fix, output, text};

fn diagnostic_for(circuit: &Circuit, code: &str) -> qctidy_facade::Diagnostic {
    Session::default()
        .check(circuit)
        .diagnostics()
        .iter()
        .find(|diagnostic| *diagnostic.metadata().code() == code)
        .cloned()
        .expect("the rule should be detected")
}

#[test]
fn fix_replaces_double_s_with_a_z() {
    let circuit = Circuit::from_operations(vec![GateOperation::s(0), GateOperation::s(0)]);
    let diagnostic = diagnostic_for(&circuit, "P001");
    let source_map = vec![
        SourceLocation {
            index: 0,
            line: 0,
            column: 0,
            end_line: 0,
            end_column: 7,
        },
        SourceLocation {
            index: 1,
            line: 1,
            column: 0,
            end_line: 1,
            end_column: 7,
        },
    ];

    let source = "qc.s(0)\nqc.s(0)\n";
    let edits = fix::edits_for(
        Arc::from(source),
        &diagnostic,
        Arc::from(source_map),
        Arc::from("qc"),
        &FixContext::from_source(source),
    )
    .expect("P001 should have a source fix");

    assert_eq!(fix::apply_edits(source, &edits), "qc.z(0)\n");
}

#[test]
fn fix_removes_consecutive_hadamards() {
    let circuit = Circuit::from_operations(vec![GateOperation::h(0), GateOperation::h(0)]);
    let diagnostic = diagnostic_for(&circuit, "R002");
    let source_map = vec![
        SourceLocation {
            index: 0,
            line: 0,
            column: 0,
            end_line: 0,
            end_column: 7,
        },
        SourceLocation {
            index: 1,
            line: 1,
            column: 0,
            end_line: 1,
            end_column: 7,
        },
    ];

    let source = "qc.h(0)\nqc.h(0)\n";
    let edits = fix::edits_for(
        Arc::from(source),
        &diagnostic,
        Arc::from(source_map),
        Arc::from("qc"),
        &FixContext::from_source(source),
    )
    .expect("R002 should have a source fix");

    assert_eq!(fix::apply_edits(source, &edits), "\n");
}

#[test]
fn canonicalization_rules_have_no_fix() {
    let circuit = Circuit::from_operations(vec![GateOperation::x(0), GateOperation::x(2)]);
    let session = Session::default();
    let diagnostic = session
        .check(&circuit)
        .diagnostics()
        .iter()
        .find(|diagnostic| diagnostic.metadata().category().code() == "C")
        .cloned()
        .expect("a canonicalization diagnostic should be detected");

    let source_map = vec![SourceLocation {
        index: 0,
        line: 0,
        column: 0,
        end_line: 0,
        end_column: 7,
    }];

    assert!(
        fix::edits_for(
            Arc::from(""),
            &diagnostic,
            Arc::from(source_map),
            Arc::from("qc"),
            &FixContext::from_source(""),
        )
        .is_none()
    );
}

#[test]
fn fix_indents_multi_gate_replacements() {
    let circuit = Circuit::from_operations(vec![
        GateOperation::try_cx(1, 2).expect("distinct qubits"),
        GateOperation::try_cx(0, 1).expect("distinct qubits"),
        GateOperation::try_cx(0, 2).expect("distinct qubits"),
    ]);
    let diagnostic = diagnostic_for(&circuit, "X003");
    let source_map = vec![
        SourceLocation {
            index: 0,
            line: 0,
            column: 4,
            end_line: 0,
            end_column: 15,
        },
        SourceLocation {
            index: 1,
            line: 1,
            column: 4,
            end_line: 1,
            end_column: 15,
        },
        SourceLocation {
            index: 2,
            line: 2,
            column: 4,
            end_line: 2,
            end_column: 15,
        },
    ];

    let source = "    qc.cx(1, 2)\n    qc.cx(0, 1)\n    qc.cx(0, 2)\n";
    let edits = fix::edits_for(
        Arc::from(source),
        &diagnostic,
        Arc::from(source_map),
        Arc::from("qc"),
        &FixContext::from_source(source),
    )
    .expect("X003 should have a source fix");

    assert_eq!(
        fix::apply_edits(source, &edits),
        "    qc.cx(0, 1)\n    qc.cx(1, 2)\n"
    );
}

#[test]
fn fix_preserves_crlf_line_endings() {
    let circuit = Circuit::from_operations(vec![
        GateOperation::try_cx(1, 2).expect("distinct qubits"),
        GateOperation::try_cx(0, 1).expect("distinct qubits"),
        GateOperation::try_cx(0, 2).expect("distinct qubits"),
    ]);
    let diagnostic = diagnostic_for(&circuit, "X003");
    let source_map = vec![
        SourceLocation {
            index: 0,
            line: 0,
            column: 4,
            end_line: 0,
            end_column: 15,
        },
        SourceLocation {
            index: 1,
            line: 1,
            column: 4,
            end_line: 1,
            end_column: 15,
        },
        SourceLocation {
            index: 2,
            line: 2,
            column: 4,
            end_line: 2,
            end_column: 15,
        },
    ];

    let source = "    qc.cx(1, 2)\r\n    qc.cx(0, 1)\r\n    qc.cx(0, 2)\r\n";
    let edits = fix::edits_for(
        Arc::from(source),
        &diagnostic,
        Arc::from(source_map),
        Arc::from("qc"),
        &FixContext::from_source(source),
    )
    .expect("X003 should have a source fix");
    let fixed = fix::apply_edits(source, &edits);

    assert_eq!(fixed, "    qc.cx(0, 1)\r\n    qc.cx(1, 2)\r\n");
    assert!(
        !fixed.replace("\r\n", "").contains('\n'),
        "the result should not mix line endings"
    );
}

#[test]
fn diagnostic_json_uses_utf16_columns() {
    // The leading "ñ" is two bytes but one UTF-16 code unit.
    let circuit = Circuit::from_operations(vec![GateOperation::s(0), GateOperation::s(0)]);
    let diagnostic = diagnostic_for(&circuit, "P001");
    let source_map = vec![
        SourceLocation {
            index: 0,
            line: 0,
            column: 3,
            end_line: 0,
            end_column: 10,
        },
        SourceLocation {
            index: 1,
            line: 1,
            column: 3,
            end_line: 1,
            end_column: 10,
        },
    ];
    let source = "ñ qc.s(0)\nñ qc.s(0)\n";
    let edits = fix::edits_for(
        Arc::from(source),
        &diagnostic,
        Arc::from(source_map.clone()),
        Arc::from("qc"),
        &FixContext::from_source(source),
    )
    .expect("P001 should have a source fix");

    let json = output::JsonDiagnostic::new(
        "file.py",
        Some("qc"),
        &diagnostic,
        Some(&source_map),
        Some(edits),
        &text::lines(source),
    );
    let value = serde_json::to_value(&json).expect("the diagnostic should serialize");

    assert_eq!(value["source_locations"][0]["column"], 2);
    assert_eq!(value["source_locations"][1]["column"], 2);
    assert_eq!(value["fixes"][0]["column"], 2);
    assert_eq!(value["fixes"][0]["end_column"], 9);
}

#[test]
fn fix_normalizes_an_out_of_range_angle() {
    let circuit = Circuit::from_operations(vec![
        GateOperation::try_rz(-FRAC_PI_2, 0).expect("valid angle"),
    ]);
    let diagnostic = diagnostic_for(&circuit, "C003");
    let source_map = vec![SourceLocation {
        index: 0,
        line: 0,
        column: 0,
        end_line: 0,
        end_column: 23,
    }];
    let source = "qc.rz(-numpy.pi / 2, 0)\n";
    let edits = fix::edits_for(
        Arc::from(source),
        &diagnostic,
        Arc::from(source_map),
        Arc::from("qc"),
        &FixContext::from_source(source),
    )
    .expect("C003 should have a source fix");

    assert_eq!(
        fix::apply_edits(source, &edits),
        "qc.rz(7 * numpy.pi / 2, 0)\n"
    );
}

#[test]
fn fix_uses_the_pi_identifier_of_the_source() {
    let circuit = Circuit::from_operations(vec![
        GateOperation::try_rz(-FRAC_PI_2, 0).expect("valid angle"),
    ]);
    let diagnostic = diagnostic_for(&circuit, "C003");
    let source_map = vec![SourceLocation {
        index: 0,
        line: 0,
        column: 0,
        end_line: 0,
        end_column: 20,
    }];
    let source = "qc.rz(-np.pi / 2, 0)\n";
    let edits = fix::edits_for(
        Arc::from(source),
        &diagnostic,
        Arc::from(source_map),
        Arc::from("qc"),
        &FixContext::from_source(source),
    )
    .expect("C003 should have a source fix");

    assert_eq!(
        fix::apply_edits(source, &edits),
        "qc.rz(7 * np.pi / 2, 0)\n"
    );
}

#[test]
fn fix_is_skipped_without_a_pi_binding() {
    let circuit =
        Circuit::from_operations(vec![GateOperation::try_rz(-PI, 0).expect("valid angle")]);
    let diagnostic = diagnostic_for(&circuit, "C003");
    let source_map = vec![SourceLocation {
        index: 0,
        line: 0,
        column: 0,
        end_line: 0,
        end_column: 28,
    }];
    let source = "qc.rz(-3.141592653589793, 0)\n";

    assert!(
        fix::edits_for(
            Arc::from(source),
            &diagnostic,
            Arc::from(source_map),
            Arc::from("qc"),
            &FixContext::from_source(source),
        )
        .is_none(),
        "a replacement that needs pi must not be offered without a binding"
    );
}
