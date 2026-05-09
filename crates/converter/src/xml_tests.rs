use std::f64::consts::PI;

use qsimplify::{GateOperation, math};

use qsimplify_ports::{ConversionFormat, ParseError};

use crate::xml;

#[test]
#[expect(clippy::unwrap_used)]
fn parse_empty_list_returns_empty_vec() {
    let input = "<gates></gates>";
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert!(operations.is_empty());
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_id_from_xml() {
    let input = r#"<gates><gate type="id" qubit="0"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::ID { qubit } => assert_eq!(qubit, 0),
        _ => panic!("Expected ID gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_h_from_xml() {
    let input = r#"<gates><gate type="h" qubit="1"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::H { qubit } => assert_eq!(qubit, 1),
        _ => panic!("Expected H gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_x_from_xml() {
    let input = r#"<gates><gate type="x" qubit="2"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::X { qubit } => assert_eq!(qubit, 2),
        _ => panic!("Expected X gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_y_from_xml() {
    let input = r#"<gates><gate type="y" qubit="3"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::Y { qubit } => assert_eq!(qubit, 3),
        _ => panic!("Expected Y gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_z_from_xml() {
    let input = r#"<gates><gate type="z" qubit="4"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::Z { qubit } => assert_eq!(qubit, 4),
        _ => panic!("Expected Z gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_p_from_xml() {
    let input = r#"<gates><gate type="p" qubit="5" angle="1.5"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::P { qubit, angle } => {
            assert_eq!(qubit, 5);
            assert!(math::are_floats_equal(angle, 1.5));
        }
        _ => panic!("Expected P gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_rx_from_xml() {
    let input = r#"<gates><gate type="rx" qubit="6" angle="3.141592653589793"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::RX { qubit, angle } => {
            assert_eq!(qubit, 6);
            assert!(math::are_floats_equal(angle, PI));
        }
        _ => panic!("Expected RX gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_ry_from_xml() {
    let input = r#"<gates><gate type="ry" qubit="7" angle="0.5"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::RY { qubit, angle } => {
            assert_eq!(qubit, 7);
            assert!(math::are_floats_equal(angle, 0.5));
        }
        _ => panic!("Expected RY gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_rz_from_xml() {
    let input = r#"<gates><gate type="rz" qubit="8" angle="2.0"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::RZ { qubit, angle } => {
            assert_eq!(qubit, 8);
            assert!(math::are_floats_equal(angle, 2.0));
        }
        _ => panic!("Expected RZ gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_s_from_xml() {
    let input = r#"<gates><gate type="s" qubit="9"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::S { qubit } => assert_eq!(qubit, 9),
        _ => panic!("Expected S gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_sdg_from_xml() {
    let input = r#"<gates><gate type="sdg" qubit="10"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::SDG { qubit } => assert_eq!(qubit, 10),
        _ => panic!("Expected SDG gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_sx_from_xml() {
    let input = r#"<gates><gate type="sx" qubit="11"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::SX { qubit } => assert_eq!(qubit, 11),
        _ => panic!("Expected SX gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_sy_from_xml() {
    let input = r#"<gates><gate type="sy" qubit="12"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::SY { qubit } => assert_eq!(qubit, 12),
        _ => panic!("Expected SY gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_t_from_xml() {
    let input = r#"<gates><gate type="t" qubit="13"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::T { qubit } => assert_eq!(qubit, 13),
        _ => panic!("Expected T gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_tdg_from_xml() {
    let input = r#"<gates><gate type="tdg" qubit="14"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::TDG { qubit } => assert_eq!(qubit, 14),
        _ => panic!("Expected TDG gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_measure_from_xml() {
    let input = r#"<gates><gate type="m" qubit="15" bit="3"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::Measure { qubit, bit } => {
            assert_eq!(qubit, 15);
            assert_eq!(bit, 3);
        }
        _ => panic!("Expected Measure gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_swap_from_xml() {
    let input = r#"<gates><gate type="swap" qubit1="0" qubit2="5"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::Swap { qubit1, qubit2 } => {
            assert_eq!(qubit1, 0);
            assert_eq!(qubit2, 5);
        }
        _ => panic!("Expected Swap gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_ch_from_xml() {
    let input = r#"<gates><gate type="ch" control="2" target="7"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::CH { control, target } => {
            assert_eq!(control, 2);
            assert_eq!(target, 7);
        }
        _ => panic!("Expected CH gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_cx_from_xml() {
    let input = r#"<gates><gate type="cx" control="3" target="8"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::CX { control, target } => {
            assert_eq!(control, 3);
            assert_eq!(target, 8);
        }
        _ => panic!("Expected CX gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_cy_from_xml() {
    let input = r#"<gates><gate type="cy" control="4" target="9"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::CY { control, target } => {
            assert_eq!(control, 4);
            assert_eq!(target, 9);
        }
        _ => panic!("Expected CY gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_cz_from_xml() {
    let input = r#"<gates><gate type="cz" qubit1="1" qubit2="6"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::CZ { qubit1, qubit2 } => {
            assert_eq!(qubit1, 1);
            assert_eq!(qubit2, 6);
        }
        _ => panic!("Expected CZ gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_cp_from_xml() {
    let input = r#"<gates><gate type="cp" qubit1="5" qubit2="10" angle="0.75"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::CP {
            qubit1,
            qubit2,
            angle,
        } => {
            assert_eq!(qubit1, 5);
            assert_eq!(qubit2, 10);
            assert!(math::are_floats_equal(angle, 0.75));
        }
        _ => panic!("Expected CP gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_cswap_from_xml() {
    let input = r#"<gates><gate type="cswap" control="0" target1="1" target2="2"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::CSwap {
            control,
            target1,
            target2,
        } => {
            assert_eq!(control, 0);
            assert_eq!(target1, 1);
            assert_eq!(target2, 2);
        }
        _ => panic!("Expected CSwap gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_ccx_from_xml() {
    let input = r#"<gates><gate type="ccx" control1="0" control2="1" target="2"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::CCX {
            control1,
            control2,
            target,
        } => {
            assert_eq!(control1, 0);
            assert_eq!(control2, 1);
            assert_eq!(target, 2);
        }
        _ => panic!("Expected CCX gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_ccz_from_xml() {
    let input = r#"<gates><gate type="ccz" qubit1="3" qubit2="4" qubit3="5"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 1);

    match operations[0] {
        GateOperation::CCZ {
            qubit1,
            qubit2,
            qubit3,
        } => {
            assert_eq!(qubit1, 3);
            assert_eq!(qubit2, 4);
            assert_eq!(qubit3, 5);
        }
        _ => panic!("Expected CCZ gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_list_from_xml() {
    let input = r#"<gates><gate type="h" qubit="0"/><gate type="cx" control="0" target="1"/><gate type="m" qubit="1" bit="0"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 3);

    match operations[0] {
        GateOperation::H { qubit } => assert_eq!(qubit, 0),
        _ => panic!("Expected H gate"),
    }

    match operations[1] {
        GateOperation::CX { control, target } => {
            assert_eq!(control, 0);
            assert_eq!(target, 1);
        }
        _ => panic!("Expected CX gate"),
    }

    match operations[2] {
        GateOperation::Measure { qubit, bit } => {
            assert_eq!(qubit, 1);
            assert_eq!(bit, 0);
        }
        _ => panic!("Expected Measure gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_list_from_pretty_xml() {
    let input = r#"<gates>
  <gate type="h" qubit="0"/>
  <gate type="cx" control="0" target="1"/>
  <gate type="m" qubit="1" bit="0"/>
</gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 3);

    match operations[0] {
        GateOperation::H { qubit } => assert_eq!(qubit, 0),
        _ => panic!("Expected H gate"),
    }

    match operations[1] {
        GateOperation::CX { control, target } => {
            assert_eq!(control, 0);
            assert_eq!(target, 1);
        }
        _ => panic!("Expected CX gate"),
    }

    match operations[2] {
        GateOperation::Measure { qubit, bit } => {
            assert_eq!(qubit, 1);
            assert_eq!(bit, 0);
        }
        _ => panic!("Expected Measure gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used)]
fn parse_empty_string_fails() {
    let result = xml::parse(b"");

    assert!(result.unwrap().is_empty());
}

#[test]
#[expect(clippy::unwrap_used)]
fn parse_whitespace_only_fails() {
    let result = xml::parse(b"   \n\t  ");

    assert!(result.unwrap().is_empty());
}

#[test]
#[expect(clippy::panic)]
fn parse_wrong_outer_type_fails() {
    let result = xml::parse(b"<gate type=\"h\" qubit=\"0\"/></gate>");

    match result {
        Err(ParseError::InvalidInput { format, .. }) => {
            assert_eq!(format, ConversionFormat::Xml);
        }
        _ => panic!("Expected InvalidInput error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn parse_missing_required_field_fails() {
    let result = xml::parse(br#"<gates><gate type="cp" qubit1="2" qubit2="3"/></gates>"#);

    match result {
        Err(ParseError::MissingRequiredField {
            format,
            field,
            gate,
        }) => {
            assert_eq!(format, ConversionFormat::Xml);
            assert_eq!(field, "angle");
            assert_eq!(gate, "cp");
        }
        _ => panic!("Expected MissingRequiredField error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn parse_unknown_gate_type_fails() {
    let result = xml::parse(br#"<gates><gate type="?" qubit="0"/></gates>"#);

    match result {
        Err(ParseError::UnknownGateType { gate }) => {
            assert_eq!(gate, "?");
        }
        _ => panic!("Expected UnknownGateType error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn parse_fully_corrupted_input_fails() {
    let result = xml::parse(b"\x00\x01\x02\x03\xff\xfe");

    match result {
        Err(ParseError::InvalidInput { format, .. }) => {
            assert_eq!(format, ConversionFormat::Xml);
        }
        _ => panic!("Expected InvalidInput error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn parse_slightly_corrupted_xml_fails() {
    let result = xml::parse(b"<gates><gate type=\"h\" qubit==\"1\"/></gates>");

    match result {
        Err(ParseError::InvalidInput { format, .. }) => {
            assert_eq!(format, ConversionFormat::Xml);
        }
        _ => panic!("Expected InvalidInput error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn parse_missing_gate_type_field_fails() {
    let result = xml::parse(br#"<gates><gate qubit="0"/></gates>"#);

    match result {
        Err(ParseError::UnknownGateType { gate }) => {
            assert_eq!(gate, "");
        }
        _ => panic!("Expected UnknownGateType error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn parse_wrong_field_type_fails() {
    let result = xml::parse(br#"<gates><gate type="h" qubit="not_a_number"/></gates>"#);

    match result {
        Err(ParseError::MissingRequiredField {
            format,
            field,
            gate,
        }) => {
            assert_eq!(format, ConversionFormat::Xml);
            assert_eq!(field, "qubit");
            assert_eq!(gate, "h");
        }
        _ => panic!("Expected MissingRequiredField error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::unwrap_used)]
fn parse_null_input_fails() {
    let result = xml::parse(b"");

    assert!(result.unwrap().is_empty());
}

#[test]
#[expect(clippy::unwrap_used)]
fn parse_string_instead_of_document_fails() {
    let result = xml::parse(b"this is a string");

    assert!(result.unwrap().is_empty());
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_empty_list_to_xml() {
    let result = xml::serialize(&[], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_id_to_xml() {
    let operation = GateOperation::id(0);
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected =
        "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"id\" qubit=\"0\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_h_to_xml() {
    let operation = GateOperation::h(1);
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected =
        "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"h\" qubit=\"1\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_x_to_xml() {
    let operation = GateOperation::x(2);
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected =
        "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"x\" qubit=\"2\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_y_to_xml() {
    let operation = GateOperation::y(3);
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected =
        "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"y\" qubit=\"3\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_z_to_xml() {
    let operation = GateOperation::z(4);
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected =
        "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"z\" qubit=\"4\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_p_to_xml() {
    let operation = GateOperation::try_p(1.5, 5).unwrap();
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"p\" qubit=\"5\" angle=\"1.5\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_rx_to_xml() {
    let operation = GateOperation::try_rx(PI, 6).unwrap();
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"rx\" qubit=\"6\" angle=\"3.141592653589793\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_ry_to_xml() {
    let operation = GateOperation::try_ry(0.5, 7).unwrap();
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"ry\" qubit=\"7\" angle=\"0.5\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_rz_to_xml() {
    let operation = GateOperation::try_rz(2.0, 8).unwrap();
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"rz\" qubit=\"8\" angle=\"2\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_s_to_xml() {
    let operation = GateOperation::s(9);
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected =
        "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"s\" qubit=\"9\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_sdg_to_xml() {
    let operation = GateOperation::sdg(10);
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"sdg\" qubit=\"10\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_sx_to_xml() {
    let operation = GateOperation::sx(11);
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected =
        "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"sx\" qubit=\"11\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_sy_to_xml() {
    let operation = GateOperation::sy(12);
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected =
        "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"sy\" qubit=\"12\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_t_to_xml() {
    let operation = GateOperation::t(13);
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected =
        "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"t\" qubit=\"13\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_tdg_to_xml() {
    let operation = GateOperation::tdg(14);
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"tdg\" qubit=\"14\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_measure_to_xml() {
    let operation = GateOperation::measure(15, 3);
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"m\" qubit=\"15\" bit=\"3\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_swap_to_xml() {
    let operation = GateOperation::try_swap(0, 5).unwrap();
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"swap\" qubit1=\"0\" qubit2=\"5\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_ch_to_xml() {
    let operation = GateOperation::try_ch(2, 7).unwrap();
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"ch\" control=\"2\" target=\"7\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_cx_to_xml() {
    let operation = GateOperation::try_cx(3, 8).unwrap();
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"cx\" control=\"3\" target=\"8\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_cy_to_xml() {
    let operation = GateOperation::try_cy(4, 9).unwrap();
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"cy\" control=\"4\" target=\"9\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_cz_to_xml() {
    let operation = GateOperation::try_cz(1, 6).unwrap();
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"cz\" qubit1=\"1\" qubit2=\"6\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_cp_to_xml() {
    let operation = GateOperation::try_cp(0.75, 5, 10).unwrap();
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"cp\" qubit1=\"5\" qubit2=\"10\" angle=\"0.75\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_cswap_to_xml() {
    let operation = GateOperation::try_c_swap(0, 1, 2).unwrap();
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"cswap\" control=\"0\" target1=\"1\" target2=\"2\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_ccx_to_xml() {
    let operation = GateOperation::try_ccx(0, 1, 2).unwrap();
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"ccx\" control1=\"0\" control2=\"1\" target=\"2\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_ccz_to_xml() {
    let operation = GateOperation::try_ccz(3, 4, 5).unwrap();
    let result = xml::serialize(&[operation], false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"ccz\" qubit1=\"3\" qubit2=\"4\" qubit3=\"5\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_list_to_xml() {
    let operations = vec![
        GateOperation::h(0),
        GateOperation::try_cx(0, 1).unwrap(),
        GateOperation::measure(1, 0),
    ];
    let result = xml::serialize(&operations, false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?><gates><gate type=\"h\" qubit=\"0\"/><gate type=\"cx\" control=\"0\" target=\"1\"/><gate type=\"m\" qubit=\"1\" bit=\"0\"/></gates>";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_list_to_pretty_xml() {
    let operations = vec![
        GateOperation::h(0),
        GateOperation::try_cx(0, 1).unwrap(),
        GateOperation::measure(1, 0),
    ];
    let result = xml::serialize(&operations, true, 2).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = r#"<?xml version="1.0" encoding="utf-8"?>
<gates>
  <gate type="h" qubit="0"/>
  <gate type="cx" control="0" target="1"/>
  <gate type="m" qubit="1" bit="0"/>
</gates>"#;

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn parse_then_serialize_preserves_data() {
    let input = r#"<gates><gate type="h" qubit="0"/><gate type="p" qubit="3" angle="1.234"/><gate type="cx" control="0" target="1"/><gate type="m" qubit="1" bit="0"/></gates>"#;
    let operations = xml::parse(input.as_bytes()).unwrap();

    assert_eq!(operations.len(), 4);

    match operations[0] {
        GateOperation::H { qubit } => assert_eq!(qubit, 0),
        _ => panic!("Expected H gate"),
    }

    match operations[1] {
        GateOperation::P { qubit, angle } => {
            assert_eq!(qubit, 3);
            assert!(math::are_floats_equal(angle, 1.234),);
        }
        _ => panic!("Expected P gate"),
    }

    match operations[2] {
        GateOperation::CX { control, target } => {
            assert_eq!(control, 0);
            assert_eq!(target, 1);
        }
        _ => panic!("Expected CX gate"),
    }

    match operations[3] {
        GateOperation::Measure { qubit, bit } => {
            assert_eq!(qubit, 1);
            assert_eq!(bit, 0);
        }
        _ => panic!("Expected Measure gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used)]
fn serialize_to_xml_preserves_data() {
    let operations = vec![
        GateOperation::h(0),
        GateOperation::try_p(1.234, 3).unwrap(),
        GateOperation::try_cx(0, 1).unwrap(),
        GateOperation::measure(1, 0),
    ];

    let result = xml::serialize(&operations, false, 0).unwrap();
    let actual = String::from_utf8(result).unwrap();
    let expected = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\
<gates>\
<gate type=\"h\" qubit=\"0\"/>\
<gate type=\"p\" qubit=\"3\" angle=\"1.234\"/>\
<gate type=\"cx\" control=\"0\" target=\"1\"/>\
<gate type=\"m\" qubit=\"1\" bit=\"0\"/>\
</gates>";

    assert_eq!(actual, expected);
}
