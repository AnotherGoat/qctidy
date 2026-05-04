use std::f64::consts::PI;

use qsimplify::dto::GateOperation;
use qsimplify_ports::{ConversionFormat, ParseError};

use crate::message_pack;

#[test]
#[expect(clippy::unwrap_used)]
fn msgpack_round_trip_empty_list() {
    let serialized = message_pack::serialize(&[]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert!(parsed.is_empty());
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_id() {
    let operation = GateOperation::id(0);
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::ID { qubit } => assert_eq!(qubit, 0),
        _ => panic!("Expected ID gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_h() {
    let operation = GateOperation::h(1);
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::H { qubit } => assert_eq!(qubit, 1),
        _ => panic!("Expected H gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_x() {
    let operation = GateOperation::x(2);
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::X { qubit } => assert_eq!(qubit, 2),
        _ => panic!("Expected X gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_y() {
    let operation = GateOperation::y(3);
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::Y { qubit } => assert_eq!(qubit, 3),
        _ => panic!("Expected Y gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_z() {
    let operation = GateOperation::z(4);
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::Z { qubit } => assert_eq!(qubit, 4),
        _ => panic!("Expected Z gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_p() {
    let operation = GateOperation::try_p(1.5, 5).unwrap();
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::P { qubit, angle } => {
            assert_eq!(qubit, 5);
            assert!((angle - 1.5).abs() < f64::EPSILON);
        }
        _ => panic!("Expected P gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_rx() {
    let operation = GateOperation::try_rx(PI, 6).unwrap();
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::RX { qubit, angle } => {
            assert_eq!(qubit, 6);
            assert!((angle - PI).abs() < f64::EPSILON);
        }
        _ => panic!("Expected RX gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_ry() {
    let operation = GateOperation::try_ry(0.5, 7).unwrap();
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::RY { qubit, angle } => {
            assert_eq!(qubit, 7);
            assert!((angle - 0.5).abs() < f64::EPSILON);
        }
        _ => panic!("Expected RY gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_rz() {
    let operation = GateOperation::try_rz(2.0, 8).unwrap();
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::RZ { qubit, angle } => {
            assert_eq!(qubit, 8);
            assert!((angle - 2.0).abs() < f64::EPSILON);
        }
        _ => panic!("Expected RZ gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_s() {
    let operation = GateOperation::s(9);
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::S { qubit } => assert_eq!(qubit, 9),
        _ => panic!("Expected S gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_sdg() {
    let operation = GateOperation::sdg(10);
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::SDG { qubit } => assert_eq!(qubit, 10),
        _ => panic!("Expected SDG gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_sx() {
    let operation = GateOperation::sx(11);
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::SX { qubit } => assert_eq!(qubit, 11),
        _ => panic!("Expected SX gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn round_trop_sy() {
    let operation = GateOperation::sy(12);
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::SY { qubit } => assert_eq!(qubit, 12),
        _ => panic!("Expected SY gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_t() {
    let operation = GateOperation::t(13);
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::T { qubit } => assert_eq!(qubit, 13),
        _ => panic!("Expected T gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_tdg() {
    let operation = GateOperation::tdg(14);
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::TDG { qubit } => assert_eq!(qubit, 14),
        _ => panic!("Expected TDG gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_measure() {
    let operation = GateOperation::measure(15, 3);
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::Measure { qubit, bit } => {
            assert_eq!(qubit, 15);
            assert_eq!(bit, 3);
        }
        _ => panic!("Expected Measure gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_swap() {
    let operation = GateOperation::try_swap(0, 5).unwrap();
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::Swap { qubit1, qubit2 } => {
            assert_eq!(qubit1, 0);
            assert_eq!(qubit2, 5);
        }
        _ => panic!("Expected Swap gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_ch() {
    let operation = GateOperation::try_ch(2, 7).unwrap();
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::CH { control, target } => {
            assert_eq!(control, 2);
            assert_eq!(target, 7);
        }
        _ => panic!("Expected CH gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_cx() {
    let operation = GateOperation::try_cx(3, 8).unwrap();
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::CX { control, target } => {
            assert_eq!(control, 3);
            assert_eq!(target, 8);
        }
        _ => panic!("Expected CX gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_cy() {
    let operation = GateOperation::try_cy(4, 9).unwrap();
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::CY { control, target } => {
            assert_eq!(control, 4);
            assert_eq!(target, 9);
        }
        _ => panic!("Expected CY gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_cz() {
    let operation = GateOperation::try_cz(1, 6).unwrap();
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::CZ { qubit1, qubit2 } => {
            assert_eq!(qubit1, 1);
            assert_eq!(qubit2, 6);
        }
        _ => panic!("Expected CZ gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_cp() {
    let operation = GateOperation::try_cp(0.75, 5, 10).unwrap();
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
        GateOperation::CP {
            qubit1,
            qubit2,
            angle,
        } => {
            assert_eq!(qubit1, 5);
            assert_eq!(qubit2, 10);
            assert!((angle - 0.75).abs() < f64::EPSILON);
        }
        _ => panic!("Expected CP gate"),
    }
}

#[test]
#[expect(clippy::unwrap_used, clippy::panic)]
fn msgpack_round_trip_cswap() {
    let operation = GateOperation::try_c_swap(0, 1, 2).unwrap();
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
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
fn msgpack_round_trip_ccx() {
    let operation = GateOperation::try_ccx(0, 1, 2).unwrap();
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
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
fn msgpack_round_trip_ccz() {
    let operation = GateOperation::try_ccz(3, 4, 5).unwrap();
    let serialized = message_pack::serialize(&[operation]).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 1);

    match parsed[0] {
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
fn msgpack_round_trip_list() {
    let operations = vec![
        GateOperation::h(0),
        GateOperation::try_cx(0, 1).unwrap(),
        GateOperation::measure(1, 0),
    ];

    let serialized = message_pack::serialize(&operations).unwrap();
    let parsed = message_pack::parse(&serialized).unwrap();

    assert_eq!(parsed.len(), 3);

    match parsed[0] {
        GateOperation::H { qubit } => assert_eq!(qubit, 0),
        _ => panic!("Expected H gate"),
    }

    match parsed[1] {
        GateOperation::CX { control, target } => {
            assert_eq!(control, 0);
            assert_eq!(target, 1);
        }
        _ => panic!("Expected CX gate"),
    }

    match parsed[2] {
        GateOperation::Measure { qubit, bit } => {
            assert_eq!(qubit, 1);
            assert_eq!(bit, 0);
        }
        _ => panic!("Expected Measure gate"),
    }
}

#[test]
#[expect(clippy::panic)]
fn parse_empty_msgpack_input_fails() {
    let result = message_pack::parse(b"");

    match result {
        Err(ParseError::InvalidInput { format, message }) => {
            assert_eq!(format, ConversionFormat::MessagePack);
            assert!(message.contains("IO error"));
        }
        _ => panic!("Expected InvalidInput error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn parse_corrupted_msgpack_input_fails() {
    let result = message_pack::parse(b"\x00\x01\x02\x03\xff\xfe");

    match result {
        Err(ParseError::InvalidInput { format, message }) => {
            assert_eq!(format, ConversionFormat::MessagePack);
            assert!(message.contains("invalid"));
        }
        _ => panic!("Expected InvalidInput error, got: {result:?}"),
    }
}
