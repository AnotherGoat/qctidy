use std::path::Path;

use qctidy_ports::ConversionFormat;

use crate::input::{format_from_extension, guess_from_contents};

#[test]
fn extension_guesses_every_supported_format() {
    assert_eq!(
        format_from_extension(Path::new("circuit.json")),
        Some(ConversionFormat::Json)
    );
    assert_eq!(
        format_from_extension(Path::new("CIRCUIT.XML")),
        Some(ConversionFormat::Xml)
    );
    assert_eq!(
        format_from_extension(Path::new("circuit.msgpack")),
        Some(ConversionFormat::MessagePack)
    );
    assert_eq!(
        format_from_extension(Path::new("circuit.messagepack")),
        Some(ConversionFormat::MessagePack)
    );
    assert_eq!(
        format_from_extension(Path::new("circuit.message_pack")),
        Some(ConversionFormat::MessagePack)
    );
    assert_eq!(
        format_from_extension(Path::new("circuit.cbor")),
        Some(ConversionFormat::Cbor)
    );
    assert_eq!(format_from_extension(Path::new("circuit.txt")), None);
    assert_eq!(format_from_extension(Path::new("circuit")), None);
}

#[test]
fn contents_guess_json_and_xml() {
    assert_eq!(
        guess_from_contents(b"  \n {\"version\":1}"),
        Some(ConversionFormat::Json)
    );
    assert_eq!(
        guess_from_contents(b"[{\"gate\":\"h\"}]"),
        Some(ConversionFormat::Json)
    );
    assert_eq!(
        guess_from_contents(b"<?xml version=\"1.0\"?><circuit/>"),
        Some(ConversionFormat::Xml)
    );
    assert_eq!(guess_from_contents(b"\x81\xa1a\x01"), None);
}
