use std::str::FromStr;

use crate::GateType::{self, *};

#[test]
fn to_string_roundtrip_id() {
    let original = ID;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_h() {
    let original = H;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_x() {
    let original = X;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_y() {
    let original = Y;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_z() {
    let original = Z;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_p() {
    let original = P;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_rx() {
    let original = RX;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_ry() {
    let original = RY;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_rz() {
    let original = RZ;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_s() {
    let original = S;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_sdg() {
    let original = SDG;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_sx() {
    let original = SX;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_sy() {
    let original = SY;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_t() {
    let original = T;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_tdg() {
    let original = TDG;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_measure() {
    let original = Measure;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_swap() {
    let original = Swap;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_ch() {
    let original = CH;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_cx() {
    let original = CX;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_cy() {
    let original = CY;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_cz() {
    let original = CZ;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_cp() {
    let original = CP;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_cswap() {
    let original = CSwap;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_ccx() {
    let original = CCX;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_roundtrip_ccz() {
    let original = CCZ;
    let string = original.to_string();
    let parsed = GateType::from_str(&string).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn to_string_aliases() {
    assert_eq!(GateType::from_str("i").unwrap(), ID);
    assert_eq!(GateType::from_str("id").unwrap(), ID);
    assert_eq!(GateType::from_str("identity").unwrap(), ID);

    assert_eq!(GateType::from_str("hadamard").unwrap(), H);

    assert_eq!(GateType::from_str("not").unwrap(), X);

    assert_eq!(GateType::from_str("phase").unwrap(), P);

    assert_eq!(GateType::from_str("sz").unwrap(), S);
    assert_eq!(GateType::from_str("sqrtz").unwrap(), S);

    assert_eq!(GateType::from_str("sd").unwrap(), SDG);
    assert_eq!(GateType::from_str("szd").unwrap(), SDG);
    assert_eq!(GateType::from_str("szdg").unwrap(), SDG);
    assert_eq!(GateType::from_str("sqrtzd").unwrap(), SDG);
    assert_eq!(GateType::from_str("sqrtzdg").unwrap(), SDG);

    assert_eq!(GateType::from_str("sqrtx").unwrap(), SX);

    assert_eq!(GateType::from_str("sqrty").unwrap(), SY);

    assert_eq!(GateType::from_str("td").unwrap(), TDG);

    assert_eq!(GateType::from_str("measure").unwrap(), Measure);

    assert_eq!(GateType::from_str("cnot").unwrap(), CX);

    assert_eq!(GateType::from_str("cphase").unwrap(), CP);

    assert_eq!(GateType::from_str("fredkin").unwrap(), CSwap);

    assert_eq!(GateType::from_str("ccnot").unwrap(), CCX);
    assert_eq!(GateType::from_str("toffoli").unwrap(), CCX);
}

#[test]
fn to_string_unknown_returns_error() {
    GateType::from_str("unknown").unwrap_err();
    GateType::from_str("???").unwrap_err();
    GateType::from_str("").unwrap_err();
    GateType::from_str("xyz gate").unwrap_err();
}

#[test]
fn to_string_case_insensitive() {
    assert_eq!(GateType::from_str("H").unwrap(), H);
    assert_eq!(GateType::from_str("HADAMARD").unwrap(), H);
    assert_eq!(GateType::from_str("Hadamard").unwrap(), H);
    assert_eq!(GateType::from_str("cx").unwrap(), CX);
    assert_eq!(GateType::from_str("CX").unwrap(), CX);
    assert_eq!(GateType::from_str("Cnot").unwrap(), CX);
}
