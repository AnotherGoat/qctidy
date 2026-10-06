use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use qctidy_ports::ConversionFormat;

/// The place where a circuit is read from.
#[derive(Debug, Clone)]
enum Source {
    Stdin,
    File(PathBuf),
}

/// A circuit source and its display name.
#[derive(Debug, Clone)]
pub(crate) struct Input {
    source: Source,
    name: String,
}

impl Input {
    /// Resolve the input paths, using standard input when no path is given.
    #[must_use]
    pub(crate) fn resolve(paths: &[PathBuf]) -> Vec<Self> {
        if paths.is_empty() {
            return vec![Self::stdin()];
        }

        paths.iter().map(|path| Self::from_path(path)).collect()
    }

    /// Resolve a single optional path, using standard input when omitted.
    #[must_use]
    pub(crate) fn resolve_one(path: Option<&Path>) -> Self {
        path.map_or_else(Self::stdin, Self::from_path)
    }

    fn from_path(path: &Path) -> Self {
        if path.to_str() == Some("-") {
            Self::stdin()
        } else {
            Self {
                source: Source::File(path.to_path_buf()),
                name: path.display().to_string(),
            }
        }
    }

    fn stdin() -> Self {
        Self {
            source: Source::Stdin,
            name: "<stdin>".to_owned(),
        }
    }

    /// The display name of this input.
    #[must_use]
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// Read the whole input into memory.
    pub(crate) fn read(&self) -> io::Result<Vec<u8>> {
        match &self.source {
            Source::Stdin => {
                let mut bytes = Vec::new();
                io::stdin().lock().read_to_end(&mut bytes)?;
                Ok(bytes)
            }
            Source::File(path) => fs::read(path),
        }
    }

    /// The format to parse this input with.
    ///
    /// An explicit format always wins. Otherwise, it is guessed from the file
    /// extension, or from the contents when reading standard input.
    #[must_use]
    pub(crate) fn format(
        &self,
        override_format: Option<ConversionFormat>,
        bytes: &[u8],
    ) -> Option<ConversionFormat> {
        override_format.or_else(|| match &self.source {
            Source::Stdin => guess_from_contents(bytes),
            Source::File(path) => format_from_extension(path),
        })
    }
}

/// The lowercase name of a format, as accepted by `--input-format`.
#[must_use]
pub(crate) const fn format_name(format: ConversionFormat) -> &'static str {
    use ConversionFormat::*;

    match format {
        Json => "json",
        Xml => "xml",
        MessagePack => "msgpack",
        Cbor => "cbor",
    }
}

/// The format matching a file extension, if it is a known one.
#[must_use]
pub(crate) fn format_from_extension(path: &Path) -> Option<ConversionFormat> {
    let extension = path.extension()?.to_str()?.to_ascii_lowercase();

    match extension.as_str() {
        "json" => Some(ConversionFormat::Json),
        "xml" => Some(ConversionFormat::Xml),
        "msgpack" | "messagepack" | "message_pack" | "mpack" => Some(ConversionFormat::MessagePack),
        "cbor" => Some(ConversionFormat::Cbor),
        _ => None,
    }
}

fn guess_from_contents(bytes: &[u8]) -> Option<ConversionFormat> {
    let first = bytes
        .iter()
        .copied()
        .find(|byte| !byte.is_ascii_whitespace())?;

    match first {
        b'{' | b'[' => Some(ConversionFormat::Json),
        b'<' => Some(ConversionFormat::Xml),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
