#[cfg(all(
    feature = "analyzer",
    any(
        feature = "converter-cbor",
        feature = "converter-json",
        feature = "converter-msgpack",
        feature = "converter-xml",
    ),
))]
pub(crate) mod analyze_circuit;

#[cfg(all(
    feature = "analyzer",
    any(
        feature = "converter-cbor",
        feature = "converter-json",
        feature = "converter-msgpack",
        feature = "converter-xml",
    ),
))]
pub(crate) mod compare_circuits;

#[cfg(all(
    any(feature = "codegen-qiskit", feature = "codegen-openqasm3"),
    any(
        feature = "converter-cbor",
        feature = "converter-json",
        feature = "converter-msgpack",
        feature = "converter-xml",
    ),
))]
pub(crate) mod generate_code;

#[cfg(any(
    feature = "converter-cbor",
    feature = "converter-json",
    feature = "converter-msgpack",
    feature = "converter-xml",
))]
pub(crate) mod convert_circuit;

#[cfg(any(
    feature = "converter-cbor",
    feature = "converter-json",
    feature = "converter-msgpack",
    feature = "converter-xml",
))]
pub(crate) mod display_circuit;

#[cfg(all(
    feature = "presenter-graphviz",
    any(
        feature = "converter-cbor",
        feature = "converter-json",
        feature = "converter-msgpack",
        feature = "converter-xml",
    ),
))]
pub(crate) mod present_circuit;

#[cfg(any(
    feature = "converter-cbor",
    feature = "converter-json",
    feature = "converter-msgpack",
    feature = "converter-xml",
))]
pub(crate) mod simplify_circuit;
