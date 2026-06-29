use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use utoipa::ToSchema;

pub(crate) fn example_circuit() -> serde_json::Value {
    json!({
        "version": 1,
        "qubit_count": 3,
        "operations": [
            { "gate": "h", "qubit": 0 },
            { "gate": "rx", "theta": FRAC_PI_2, "qubit": 1 },
            {
                "gate": "u",
                "theta": PI,
                "phi": FRAC_PI_2,
                "lambda": FRAC_PI_4,
                "qubit": 2
            },
            { "gate": "cx", "control": 0, "target": 1 },
            { "gate": "ccx", "control1": 0, "control2": 1, "target": 2 },
            { "gate": "swap", "qubit1": 1, "qubit2": 2 },
            { "gate": "m", "qubit": 2, "bit": 0 }
        ]
    })
}

pub(crate) fn example_comparison_circuit() -> serde_json::Value {
    json!({
        "version": 1,
        "qubit_count": 3,
        "operations": [
            { "gate": "h", "qubit": 0 },
            { "gate": "x", "qubit": 1 },
            { "gate": "cx", "control": 0, "target": 2 },
            { "gate": "m", "qubit": 2, "bit": 0 }
        ]
    })
}

fn example_base64_circuit() -> serde_json::Value {
    json!({
        "encoding": "base64",
        "data": "o2d2ZXJzaW9uAWtxdWJpdF9jb3VudANqb3BlcmF0aW9uc4A="
    })
}

fn example_circuit_payload() -> serde_json::Value {
    example_circuit()
}

const fn example_pi() -> f64 {
    PI
}

const fn example_frac_pi_2() -> f64 {
    FRAC_PI_2
}

const fn example_frac_pi_4() -> f64 {
    FRAC_PI_4
}

#[derive(Deserialize, Serialize, ToSchema)]
#[schema(description = "QSimplify JSON circuit representation.", example = example_circuit)]
pub(crate) struct Circuit {
    #[schema(example = 1)]
    version: Option<u16>,
    #[schema(example = 2)]
    qubit_count: usize,
    operations: Vec<GateOperation>,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[schema(description = "One gate operation in a QSimplify JSON circuit.")]
pub(crate) struct GateOperation {
    #[schema(value_type = String, example = "h")]
    gate: GateName,
    #[schema(example = 0)]
    qubit: Option<usize>,
    #[schema(example = 0)]
    qubit1: Option<usize>,
    #[schema(example = 1)]
    qubit2: Option<usize>,
    #[schema(example = 2)]
    qubit3: Option<usize>,
    #[schema(example = 0)]
    control: Option<usize>,
    #[schema(example = 0)]
    control1: Option<usize>,
    #[schema(example = 1)]
    control2: Option<usize>,
    #[schema(example = 1)]
    target: Option<usize>,
    #[schema(example = 1)]
    target1: Option<usize>,
    #[schema(example = 2)]
    target2: Option<usize>,
    #[schema(example = example_pi)]
    theta: Option<f64>,
    #[schema(example = example_frac_pi_2)]
    phi: Option<f64>,
    #[schema(example = example_frac_pi_4)]
    lambda: Option<f64>,
    #[schema(example = 0)]
    bit: Option<usize>,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
#[schema(
    description = "Canonical gate names accepted by the JSON circuit format. Some gates also accept aliases, such as `cnot` for `cx`."
)]
pub(crate) enum GateName {
    Id,
    H,
    X,
    Y,
    Z,
    P,
    Rx,
    Ry,
    Rz,
    S,
    Sdg,
    Sx,
    Sy,
    T,
    Tdg,
    U,
    M,
    Swap,
    Ch,
    Cx,
    Cy,
    Cz,
    Cp,
    Cswap,
    Ccx,
    Ccz,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[schema(
    description = "Base64 envelope used when converting binary formats such as CBOR and MessagePack.",
    example = example_base64_circuit
)]
pub(crate) struct Base64Circuit {
    #[schema(example = "base64")]
    encoding: String,
    #[schema(example = "oWZ2ZXJzaW9uAWtxdWJpdF9jb3VudAJqb3BlcmF0aW9uc4A=")]
    data: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(untagged)]
#[schema(
    description = "Circuit payload. Use `Circuit` for JSON format and `Base64Circuit` for binary formats.",
    example = example_circuit_payload
)]
pub(crate) enum CircuitPayload {
    Json(Circuit),
    Binary(Base64Circuit),
}

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ConversionFormatName {
    Json,
    Xml,
    MessagePack,
    Msgpack,
    Cbor,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub(crate) enum DisplayFormatName {
    Graph,
    Grid,
    Matrix,
    Circuit,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub(crate) enum PiFormatName {
    Lowercase,
    Uppercase,
    Fancy,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub(crate) enum DiracFormatName {
    Ascii,
    Fancy,
    None,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub(crate) enum CodeGenerationTargetName {
    Qiskit,
    Openqasm3,
}

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PresentationFormatName {
    GraphvizPng,
    Png,
    GraphvizGv,
    Gv,
    GraphvizSvg,
    Svg,
}
