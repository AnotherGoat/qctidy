use std::path::{Path, PathBuf};

use qctidy_ports::ConversionFormat;

use crate::input::{Input, format_from_extension, guess_from_contents};

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

#[test]
fn stdin_with_python_name_is_python() {
    let stdin_named_py = Input::resolve(&[])
        .into_iter()
        .next()
        .expect("stdin should resolve")
        .named(Some("circuit.py"));
    assert!(stdin_named_py.is_python());

    let file_py = Input::resolve(&[PathBuf::from("circuit.py")])
        .into_iter()
        .next()
        .expect("file should resolve");
    assert!(file_py.is_python());

    let stdin_unnamed = Input::resolve(&[])
        .into_iter()
        .next()
        .expect("stdin should resolve");
    assert!(!stdin_unnamed.is_python());
}
