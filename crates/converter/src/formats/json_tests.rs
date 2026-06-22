#![allow(clippy::wildcard_enum_match_arm)]

use std::f64::consts::PI;

use qsimplify::{Circuit, GateOperation, math};

use qsimplify_ports::{ConversionFormat, ParseError};

use crate::json;

#[test]
fn parse_empty_list_returns_empty_vec() {
    let input = r#"{"version":1,"qubit_count":0,"operations":[]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert!(circuit.operations().is_empty());
}

#[test]
fn parse_id_from_json() {
    let input = r#"{"version":1,"qubit_count":1,"operations":[{"gate":"id","qubit":0}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::ID { qubit } => assert_eq!(qubit, 0),
        _ => panic!("Expected ID gate"),
    }
}

#[test]
fn parse_h_from_json() {
    let input = r#"{"version":1,"qubit_count":2,"operations":[{"gate":"h","qubit":1}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::H { qubit } => assert_eq!(qubit, 1),
        _ => panic!("Expected H gate"),
    }
}

#[test]
fn parse_x_from_json() {
    let input = r#"{"version":1,"qubit_count":3,"operations":[{"gate":"x","qubit":2}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::X { qubit } => assert_eq!(qubit, 2),
        _ => panic!("Expected X gate"),
    }
}

#[test]
fn parse_y_from_json() {
    let input = r#"{"version":1,"qubit_count":4,"operations":[{"gate":"y","qubit":3}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::Y { qubit } => assert_eq!(qubit, 3),
        _ => panic!("Expected Y gate"),
    }
}

#[test]
fn parse_z_from_json() {
    let input = r#"{"version":1,"qubit_count":5,"operations":[{"gate":"z","qubit":4}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::Z { qubit } => assert_eq!(qubit, 4),
        _ => panic!("Expected Z gate"),
    }
}

#[test]
fn parse_p_from_json() {
    let input =
        r#"{"version":1,"qubit_count":6,"operations":[{"gate":"p","qubit":5,"theta":1.5}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::P { qubit, theta } => {
            assert_eq!(qubit, 5);
            assert!(math::are_floats_equal(theta, 1.5));
        }
        _ => panic!("Expected P gate"),
    }
}

#[test]
fn parse_rx_from_json() {
    let input = r#"{"version":1,"qubit_count":7,"operations":[{"gate":"rx","qubit":6,"theta":3.141592653589793}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::RX { qubit, theta } => {
            assert_eq!(qubit, 6);
            assert!(math::are_floats_equal(theta, PI));
        }
        _ => panic!("Expected RX gate"),
    }
}

#[test]
fn parse_ry_from_json() {
    let input =
        r#"{"version":1,"qubit_count":8,"operations":[{"gate":"ry","qubit":7,"theta":0.5}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::RY { qubit, theta } => {
            assert_eq!(qubit, 7);
            assert!(math::are_floats_equal(theta, 0.5));
        }
        _ => panic!("Expected RY gate"),
    }
}

#[test]
fn parse_rz_from_json() {
    let input = r#"{"version":1,"qubit_count":9,"operations":[{"gate":"rz","qubit":8,"phi":2.0}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::RZ { qubit, phi } => {
            assert_eq!(qubit, 8);
            assert!(math::are_floats_equal(phi, 2.0));
        }
        _ => panic!("Expected RZ gate"),
    }
}

#[test]
fn parse_u_from_json() {
    let input = r#"{"version":1,"qubit_count":4,"operations":[{"gate":"u","qubit":3,"theta":1.234,"phi":2.345,"lambda":3.456}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::U {
            theta,
            phi,
            lambda,
            qubit,
        } => {
            assert_eq!(qubit, 3);
            assert!(math::are_floats_equal(theta, 1.234));
            assert!(math::are_floats_equal(phi, 2.345));
            assert!(math::are_floats_equal(lambda, 3.456));
        }
        _ => panic!("Expected U gate"),
    }
}

#[test]
fn parse_s_from_json() {
    let input = r#"{"version":1,"qubit_count":10,"operations":[{"gate":"s","qubit":9}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::S { qubit } => assert_eq!(qubit, 9),
        _ => panic!("Expected S gate"),
    }
}

#[test]
fn parse_sdg_from_json() {
    let input = r#"{"version":1,"qubit_count":11,"operations":[{"gate":"sdg","qubit":10}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::SDG { qubit } => assert_eq!(qubit, 10),
        _ => panic!("Expected SDG gate"),
    }
}

#[test]
fn parse_sx_from_json() {
    let input = r#"{"version":1,"qubit_count":12,"operations":[{"gate":"sx","qubit":11}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::SX { qubit } => assert_eq!(qubit, 11),
        _ => panic!("Expected SX gate"),
    }
}

#[test]
fn parse_sy_from_json() {
    let input = r#"{"version":1,"qubit_count":13,"operations":[{"gate":"sy","qubit":12}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::SY { qubit } => assert_eq!(qubit, 12),
        _ => panic!("Expected SY gate"),
    }
}

#[test]
fn parse_t_from_json() {
    let input = r#"{"version":1,"qubit_count":14,"operations":[{"gate":"t","qubit":13}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::T { qubit } => assert_eq!(qubit, 13),
        _ => panic!("Expected T gate"),
    }
}

#[test]
fn parse_tdg_from_json() {
    let input = r#"{"version":1,"qubit_count":15,"operations":[{"gate":"tdg","qubit":14}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::TDG { qubit } => assert_eq!(qubit, 14),
        _ => panic!("Expected TDG gate"),
    }
}

#[test]
fn parse_measure_from_json() {
    let input = r#"{"version":1,"qubit_count":16,"operations":[{"gate":"m","qubit":15,"bit":3}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::Measure { qubit, bit } => {
            assert_eq!(qubit, 15);
            assert_eq!(bit, 3);
        }
        _ => panic!("Expected Measure gate"),
    }
}

#[test]
fn parse_swap_from_json() {
    let input =
        r#"{"version":1,"qubit_count":6,"operations":[{"gate":"swap","qubit1":0,"qubit2":5}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::Swap { qubit1, qubit2 } => {
            assert_eq!(qubit1, 0);
            assert_eq!(qubit2, 5);
        }
        _ => panic!("Expected Swap gate"),
    }
}

#[test]
fn parse_ch_from_json() {
    let input =
        r#"{"version":1,"qubit_count":8,"operations":[{"gate":"ch","control":2,"target":7}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::CH { control, target } => {
            assert_eq!(control, 2);
            assert_eq!(target, 7);
        }
        _ => panic!("Expected CH gate"),
    }
}

#[test]
fn parse_cx_from_json() {
    let input =
        r#"{"version":1,"qubit_count":9,"operations":[{"gate":"cx","control":3,"target":8}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::CX { control, target } => {
            assert_eq!(control, 3);
            assert_eq!(target, 8);
        }
        _ => panic!("Expected CX gate"),
    }
}

#[test]
fn parse_cy_from_json() {
    let input =
        r#"{"version":1,"qubit_count":10,"operations":[{"gate":"cy","control":4,"target":9}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::CY { control, target } => {
            assert_eq!(control, 4);
            assert_eq!(target, 9);
        }
        _ => panic!("Expected CY gate"),
    }
}

#[test]
fn parse_cz_from_json() {
    let input =
        r#"{"version":1,"qubit_count":7,"operations":[{"gate":"cz","qubit1":1,"qubit2":6}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::CZ { qubit1, qubit2 } => {
            assert_eq!(qubit1, 1);
            assert_eq!(qubit2, 6);
        }
        _ => panic!("Expected CZ gate"),
    }
}

#[test]
fn parse_cp_from_json() {
    let input = r#"{"version":1,"qubit_count":11,"operations":[{"gate":"cp","qubit1":5,"qubit2":10,"theta":0.75}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
        GateOperation::CP {
            qubit1,
            qubit2,
            theta,
        } => {
            assert_eq!(qubit1, 5);
            assert_eq!(qubit2, 10);
            assert!(math::are_floats_equal(theta, 0.75));
        }
        _ => panic!("Expected CP gate"),
    }
}

#[test]
fn parse_cswap_from_json() {
    let input = r#"{"version":1,"qubit_count":3,"operations":[{"gate":"cswap","control":0,"target1":1,"target2":2}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
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
fn parse_ccx_from_json() {
    let input = r#"{"version":1,"qubit_count":3,"operations":[{"gate":"ccx","control1":0,"control2":1,"target":2}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
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
fn parse_ccz_from_json() {
    let input = r#"{"version":1,"qubit_count":6,"operations":[{"gate":"ccz","qubit1":3,"qubit2":4,"qubit3":5}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    match circuit.operations()[0] {
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
fn parse_list_from_json() {
    let input = r#"{"version":1,"qubit_count":2,"operations":[{"gate":"h","qubit":0},{"gate":"cx","control":0,"target":1},{"gate":"m","qubit":1,"bit":0}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 3);

    match circuit.operations()[0] {
        GateOperation::H { qubit } => assert_eq!(qubit, 0),
        _ => panic!("Expected H gate"),
    }

    match circuit.operations()[1] {
        GateOperation::CX { control, target } => {
            assert_eq!(control, 0);
            assert_eq!(target, 1);
        }
        _ => panic!("Expected CX gate"),
    }

    match circuit.operations()[2] {
        GateOperation::Measure { qubit, bit } => {
            assert_eq!(qubit, 1);
            assert_eq!(bit, 0);
        }
        _ => panic!("Expected Measure gate"),
    }
}

#[test]
fn parse_list_from_pretty_json() {
    let input = r#"{
  "version": 1,
  "qubit_count": 2,
  "operations": [
    {
      "gate": "h",
      "qubit": 0
    },
    {
      "gate": "cx",
      "control": 0,
      "target": 1
    },
    {
      "gate": "m",
      "qubit": 1,
      "bit": 0
    }
  ]
}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 3);

    match circuit.operations()[0] {
        GateOperation::H { qubit } => assert_eq!(qubit, 0),
        _ => panic!("Expected H gate"),
    }

    match circuit.operations()[1] {
        GateOperation::CX { control, target } => {
            assert_eq!(control, 0);
            assert_eq!(target, 1);
        }
        _ => panic!("Expected CX gate"),
    }

    match circuit.operations()[2] {
        GateOperation::Measure { qubit, bit } => {
            assert_eq!(qubit, 1);
            assert_eq!(bit, 0);
        }
        _ => panic!("Expected Measure gate"),
    }
}

#[test]
fn parse_empty_string_fails() {
    let result = json::parse(b"");

    match result {
        Err(ParseError::InvalidInput { format, message }) => {
            assert_eq!(format, ConversionFormat::Json);
            assert!(message.contains("EOF"));
        }
        _ => panic!("Expected InvalidInput error, got: {result:?}"),
    }
}

#[test]
fn parse_whitespace_only_fails() {
    let result = json::parse(b"   \n\t  ");

    match result {
        Err(ParseError::InvalidInput { format, message }) => {
            assert_eq!(format, ConversionFormat::Json);
            assert!(message.contains("EOF"));
        }
        _ => panic!("Expected InvalidInput error, got: {result:?}"),
    }
}

#[test]
fn parse_wrong_outer_type_fails() {
    let result = json::parse(b"[]");

    match result {
        Err(ParseError::InvalidInput { format, message }) => {
            assert_eq!(format, ConversionFormat::Json);
            assert!(message.contains("expected"));
        }
        _ => panic!("Expected InvalidInput error, got: {result:?}"),
    }
}

#[test]
fn parse_missing_required_field_fails() {
    let input =
        r#"{"version":1,"qubit_count":10,"operations":[{"gate":"cp","qubit1":2,"qubit2":3}]}"#;
    let result = json::parse(input.as_bytes());

    match result {
        Err(ParseError::MissingRequiredField { field, gate }) => {
            assert_eq!(field, "theta");
            assert_eq!(gate, "cp");
        }
        _ => panic!("Expected MissingRequiredField error, got: {result:?}"),
    }
}

#[test]
fn parse_unknown_gate_type_fails() {
    let input = r#"{"version":1,"qubit_count":1,"operations":[{"gate":"?","qubit":0}]}"#;
    let result = json::parse(input.as_bytes());

    match result {
        Err(ParseError::UnknownGateType { gate }) => {
            assert_eq!(gate, "?");
        }
        _ => panic!("Expected UnknownGateType error, got: {result:?}"),
    }
}

#[test]
fn parse_unneeded_field_fails() {
    let input =
        r#"{"version":1,"qubit_count":2,"operations":[{"gate":"h","qubit":1,"what":true}]}"#;
    let result = json::parse(input.as_bytes());

    match result {
        Err(ParseError::UnknownField { field, gate }) => {
            assert_eq!(field, "what");
            assert_eq!(gate, "circuit");
        }
        _ => panic!("Expected UnknownField error, got: {result:?}"),
    }
}

#[test]
fn parse_fully_corrupted_input_fails() {
    let result = json::parse(b"\x00\x01\x02\x03\xff\xfe");

    match result {
        Err(ParseError::InvalidInput { .. }) => {}
        _ => panic!("Expected InvalidInput error, got: {result:?}"),
    }
}

#[test]
fn parse_slightly_corrupted_json_fails() {
    let result =
        json::parse(br#"{"version":1,"qubit_count":2,"operations":[{"gate":"h","qubit"::0}]}"#);

    match result {
        Err(ParseError::InvalidInput { format, message }) => {
            assert_eq!(format, ConversionFormat::Json);
            assert!(message.contains("expected"));
        }
        _ => panic!("Expected InvalidInput error, got: {result:?}"),
    }
}

#[test]
fn parse_missing_gate_field_fails() {
    let input = r#"{"version":1,"qubit_count":1,"operations":[{"qubit":0}]}"#;
    let result = json::parse(input.as_bytes());

    match result {
        Err(ParseError::InvalidInput { format, message }) => {
            assert_eq!(format, ConversionFormat::Json);
            assert!(message.contains("missing field"));
            assert!(message.contains("gate"));
        }
        _ => panic!("Expected InvalidInput error, got: {result:?}"),
    }
}

#[test]
fn parse_wrong_field_type_fails() {
    let input =
        r#"{"version":1,"qubit_count":2,"operations":[{"gate":"h","qubit":"not_a_number"}]}"#;
    let result = json::parse(input.as_bytes());

    match result {
        Err(ParseError::InvalidInput { .. }) => {}
        _ => panic!("Expected InvalidInput error, got: {result:?}"),
    }
}

#[test]
fn parse_string_instead_of_object_fails() {
    let result = json::parse(br#""this is a string""#);

    match result {
        Err(ParseError::InvalidInput { format, message }) => {
            assert_eq!(format, ConversionFormat::Json);
            assert!(message.contains("expected"));
        }
        _ => panic!("Expected InvalidInput error, got: {result:?}"),
    }
}

#[test]
fn parse_null_input_fails() {
    let result = json::parse(b"null");

    match result {
        Err(ParseError::InvalidInput { format, message }) => {
            assert_eq!(format, ConversionFormat::Json);
            assert!(message.contains("expected"));
        }
        _ => panic!("Expected InvalidInput error, got: {result:?}"),
    }
}

#[test]
fn serialize_empty_list_to_json() {
    let json = json::serialize(&Circuit::from_operations(vec![]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":0,"operations":[]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_id_to_json() {
    let operation = GateOperation::id(0);

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":1,"operations":[{"gate":"id","qubit":0}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_h_to_json() {
    let operation = GateOperation::h(1);

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":2,"operations":[{"gate":"h","qubit":1}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_x_to_json() {
    let operation = GateOperation::x(2);

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":3,"operations":[{"gate":"x","qubit":2}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_y_to_json() {
    let operation = GateOperation::y(3);

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":4,"operations":[{"gate":"y","qubit":3}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_z_to_json() {
    let operation = GateOperation::z(4);

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":5,"operations":[{"gate":"z","qubit":4}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_p_to_json() {
    let operation = GateOperation::try_p(1.5, 5).unwrap();

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected =
        r#"{"version":1,"qubit_count":6,"operations":[{"gate":"p","qubit":5,"theta":1.5}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_rx_to_json() {
    let operation = GateOperation::try_rx(PI, 6).unwrap();

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":7,"operations":[{"gate":"rx","qubit":6,"theta":3.141592653589793}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_u_to_json() {
    let operation = GateOperation::try_u(1.234, 2.345, 3.456, 3).unwrap();

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":4,"operations":[{"gate":"u","qubit":3,"theta":1.234,"phi":2.345,"lambda":3.456}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_ry_to_json() {
    let operation = GateOperation::try_ry(0.5, 7).unwrap();

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected =
        r#"{"version":1,"qubit_count":8,"operations":[{"gate":"ry","qubit":7,"theta":0.5}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_rz_to_json() {
    let operation = GateOperation::try_rz(2.0, 8).unwrap();

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected =
        r#"{"version":1,"qubit_count":9,"operations":[{"gate":"rz","qubit":8,"phi":2.0}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_s_to_json() {
    let operation = GateOperation::s(9);

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":10,"operations":[{"gate":"s","qubit":9}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_sdg_to_json() {
    let operation = GateOperation::sdg(10);

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":11,"operations":[{"gate":"sdg","qubit":10}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_sx_to_json() {
    let operation = GateOperation::sx(11);

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":12,"operations":[{"gate":"sx","qubit":11}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_sy_to_json() {
    let operation = GateOperation::sy(12);

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":13,"operations":[{"gate":"sy","qubit":12}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_t_to_json() {
    let operation = GateOperation::t(13);

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":14,"operations":[{"gate":"t","qubit":13}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_tdg_to_json() {
    let operation = GateOperation::tdg(14);

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":15,"operations":[{"gate":"tdg","qubit":14}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_measure_to_json() {
    let operation = GateOperation::measure(15, 3);

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected =
        r#"{"version":1,"qubit_count":16,"operations":[{"gate":"m","qubit":15,"bit":3}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_swap_to_json() {
    let operation = GateOperation::try_swap(0, 5).unwrap();

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected =
        r#"{"version":1,"qubit_count":6,"operations":[{"gate":"swap","qubit1":0,"qubit2":5}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_ch_to_json() {
    let operation = GateOperation::try_ch(2, 7).unwrap();

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected =
        r#"{"version":1,"qubit_count":8,"operations":[{"gate":"ch","control":2,"target":7}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_cx_to_json() {
    let operation = GateOperation::try_cx(3, 8).unwrap();

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected =
        r#"{"version":1,"qubit_count":9,"operations":[{"gate":"cx","control":3,"target":8}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_cy_to_json() {
    let operation = GateOperation::try_cy(4, 9).unwrap();

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected =
        r#"{"version":1,"qubit_count":10,"operations":[{"gate":"cy","control":4,"target":9}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_cz_to_json() {
    let operation = GateOperation::try_cz(1, 6).unwrap();

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected =
        r#"{"version":1,"qubit_count":7,"operations":[{"gate":"cz","qubit1":1,"qubit2":6}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_cp_to_json() {
    let operation = GateOperation::try_cp(0.75, 5, 10).unwrap();

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":11,"operations":[{"gate":"cp","qubit1":5,"qubit2":10,"theta":0.75}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_cswap_to_json() {
    let operation = GateOperation::try_c_swap(0, 1, 2).unwrap();

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":3,"operations":[{"gate":"cswap","control":0,"target1":1,"target2":2}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_ccx_to_json() {
    let operation = GateOperation::try_ccx(0, 1, 2).unwrap();

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":3,"operations":[{"gate":"ccx","control1":0,"control2":1,"target":2}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_ccz_to_json() {
    let operation = GateOperation::try_ccz(3, 4, 5).unwrap();

    let json = json::serialize(&Circuit::from_operations(vec![operation]), false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":6,"operations":[{"gate":"ccz","qubit1":3,"qubit2":4,"qubit3":5}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_list_to_json() {
    let circuit = Circuit::from_operations(vec![
        GateOperation::h(0),
        GateOperation::try_cx(0, 1).unwrap(),
        GateOperation::measure(1, 0),
    ]);

    let json = json::serialize(&circuit, false, 0).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{"version":1,"qubit_count":2,"operations":[{"gate":"h","qubit":0},{"gate":"cx","control":0,"target":1},{"gate":"m","qubit":1,"bit":0}]}"#;

    assert_eq!(actual, expected);
}

#[test]
fn serialize_list_to_pretty_json() {
    let circuit = Circuit::from_operations(vec![
        GateOperation::h(0),
        GateOperation::try_cx(0, 1).unwrap(),
        GateOperation::measure(1, 0),
    ]);

    let json = json::serialize(&circuit, true, 2).unwrap();
    let actual = String::from_utf8(json).unwrap();
    let expected = r#"{
  "version": 1,
  "qubit_count": 2,
  "operations": [
    {
      "gate": "h",
      "qubit": 0
    },
    {
      "gate": "cx",
      "control": 0,
      "target": 1
    },
    {
      "gate": "m",
      "qubit": 1,
      "bit": 0
    }
  ]
}"#;

    assert_eq!(actual, expected);
}

#[test]
fn parse_then_serialize_preserves_data() {
    let circuit = Circuit::from_operations(vec![
        GateOperation::h(0),
        GateOperation::try_p(1.234, 3).unwrap(),
        GateOperation::try_cx(0, 1).unwrap(),
        GateOperation::measure(1, 0),
    ]);

    let serialized = json::serialize(&circuit, false, 0).unwrap();
    let parsed = json::parse(&serialized).unwrap();

    assert_eq!(parsed.operations().len(), 4);

    match parsed.operations()[0] {
        GateOperation::H { qubit } => assert_eq!(qubit, 0),
        _ => panic!("Expected H gate"),
    }

    match parsed.operations()[1] {
        GateOperation::P { qubit, theta } => {
            assert_eq!(qubit, 3);
            assert!(math::are_floats_equal(theta, 1.234),);
        }
        _ => panic!("Expected P gate"),
    }

    match parsed.operations()[2] {
        GateOperation::CX { control, target } => {
            assert_eq!(control, 0);
            assert_eq!(target, 1);
        }
        _ => panic!("Expected CX gate"),
    }

    match parsed.operations()[3] {
        GateOperation::Measure { qubit, bit } => {
            assert_eq!(qubit, 1);
            assert_eq!(bit, 0);
        }
        _ => panic!("Expected Measure gate"),
    }
}

#[test]
fn serialize_then_parse_preserves_data() {
    let circuit = r#"{"version":1,"qubit_count":4,"operations":[{"gate":"h","qubit":0},{"gate":"p","qubit":3,"theta":1.234},{"gate":"cx","control":0,"target":1},{"gate":"m","qubit":1,"bit":0}]}"#;

    let parsed = json::parse(circuit.as_bytes()).unwrap();
    let serialized = json::serialize(&parsed, false, 0).unwrap();

    assert_eq!(String::from_utf8(serialized).unwrap(), circuit);
}

#[test]
fn parse_json_defaults_to_version_1() {
    let input = r#"{"qubit_count":2,"operations":[{"gate":"h","qubit":0}]}"#;
    let circuit = json::parse(input.as_bytes()).unwrap();

    assert_eq!(circuit.operations().len(), 1);

    let serialized = json::serialize(&circuit, false, 0).unwrap();
    let output = String::from_utf8(serialized).unwrap();

    assert!(output.contains(r#""version":1"#));
}
