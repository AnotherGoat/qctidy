use std::sync::Arc;

use getset::{CloneGetters, CopyGetters, Getters};
use newgen::New;
use qctidy::{Circuit, Graph, fixer};
use qctidy_ports::{CodeGenerationError, CodeGenerationTarget, CodegenPort, SourceLocation};

use crate::Diagnostic;

#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct FixRequest {
    #[get_clone = "pub"]
    circuit: Arc<Circuit>,
    #[get_copy = "pub"]
    iterations: u32,
}

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct FixResponse {
    #[get_clone = "pub"]
    circuit: Arc<Circuit>,
}

pub fn fix(request: &FixRequest) -> FixResponse {
    let graph: Graph = request.circuit().as_ref().into();
    let fixed = fixer::fix(graph, request.iterations());

    let circuit: Circuit = (&fixed).into();
    FixResponse::new(circuit.into())
}

/// A single source edit that applies a fix.
///
/// Positions use the same convention as [`SourceLocation`]: zero-based line and
/// byte column, with an exclusive end.
#[derive(Debug, Clone, PartialEq, Eq, Getters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct FixEdit {
    #[get_copy = "pub"]
    line: usize,
    #[get_copy = "pub"]
    column: usize,
    #[get_copy = "pub"]
    end_line: usize,
    #[get_copy = "pub"]
    end_column: usize,
    #[get = "pub"]
    new_text: String,
}

/// Request to compute the source edits for one detected diagnostic.
#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct ApplyFixRequest {
    #[get_clone = "pub"]
    source: Arc<str>,
    #[get_clone = "pub"]
    diagnostic: Diagnostic,
    #[get_clone = "pub"]
    source_map: Arc<[SourceLocation]>,
    #[get_clone = "pub"]
    circuit_name: Arc<str>,
}

/// The source edits that apply a fix, empty when the diagnostic has no fix.
#[derive(Debug, Clone, Getters, New)]
#[new(pub, const)]
#[must_use]
pub struct ApplyFixResponse {
    #[get = "pub"]
    edits: Vec<FixEdit>,
}

/// Values derived from the source text that every fix in that source shares.
///
/// Computing them once per file avoids scanning the whole source per diagnostic.
#[derive(Debug, Clone)]
#[must_use]
pub struct FixContext {
    line_ending: Arc<str>,
    pi_identifier: Option<Arc<str>>,
}

impl FixContext {
    /// Derive the context from the full source text.
    pub fn from_source(source: &str) -> Self {
        Self {
            line_ending: Arc::from(line_ending(source)),
            pi_identifier: pi_identifier(source).map(Arc::from),
        }
    }

    /// Replace the generated `numpy.pi` with the pi identifier of the source.
    fn adapt_pi(&self, text: &str) -> Option<String> {
        if !text.contains("numpy.pi") {
            return Some(text.to_owned());
        }

        let identifier = self.pi_identifier.as_deref()?;

        Some(text.replace("numpy.pi", identifier))
    }
}

/// Compute the source edits that apply the fix described by a diagnostic.
///
/// Returns an empty list when the diagnostic has no source-level fix (for
/// example, canonicalization rules).
pub fn apply_fix<C: CodegenPort>(
    request: &ApplyFixRequest,
    context: &FixContext,
    codegen: &C,
) -> Result<ApplyFixResponse, CodeGenerationError> {
    let diagnostic = request.diagnostic();
    let library_diagnostic = qctidy::Diagnostic::new(
        diagnostic.metadata(),
        diagnostic.severity(),
        diagnostic.positions().clone(),
        diagnostic.pattern_match().clone(),
        diagnostic.replacement().clone(),
    );

    let source_map = request.source_map();
    let Some(replacements) = fixer::replacement(&library_diagnostic) else {
        return Ok(ApplyFixResponse::new(Vec::new()));
    };

    let removed: Vec<&SourceLocation> = diagnostic
        .operation_indices()
        .iter()
        .filter_map(|index| source_map.get(*index))
        .collect();

    let (Some((start_line, start_column)), Some((end_line, end_column))) = (
        removed
            .iter()
            .map(|location| (location.line, location.column))
            .min(),
        removed
            .iter()
            .map(|location| (location.end_line, location.end_column))
            .max(),
    ) else {
        return Ok(ApplyFixResponse::new(Vec::new()));
    };

    let mut rendered = Vec::with_capacity(replacements.len());

    for operation in &replacements {
        rendered.push(codegen.render_operation(
            operation,
            CodeGenerationTarget::Qiskit,
            Some(request.circuit_name().as_ref()),
        )?);
    }

    let source = request.source();
    let joined = rendered.join(context.line_ending.as_ref());

    let Some(joined) = context.adapt_pi(&joined) else {
        return Ok(ApplyFixResponse::new(Vec::new()));
    };

    let new_text = reindent(source.as_ref(), start_line, start_column, &joined);

    Ok(ApplyFixResponse::new(vec![FixEdit::new(
        start_line,
        start_column,
        end_line,
        end_column,
        new_text,
    )]))
}

/// The pi identifier the source already binds, if any.
fn pi_identifier(source: &str) -> Option<&'static str> {
    ["numpy.pi", "np.pi", "math.pi"]
        .into_iter()
        .find(|identifier| source.contains(identifier))
}

/// The line ending used by the source, defaulting to LF.
fn line_ending(source: &str) -> &'static str {
    let crlf = source.matches("\r\n").count();

    if crlf > 0 && crlf * 2 >= source.matches('\n').count() {
        "\r\n"
    } else {
        "\n"
    }
}

/// Re-indent a multi-line replacement to match the indentation of the replaced region.
fn reindent(source: &str, start_line: usize, start_column: usize, new_text: &str) -> String {
    if !new_text.contains('\n') {
        return new_text.to_owned();
    }

    let indent = line_indentation(source, start_line, start_column);
    new_text.replace('\n', &format!("\n{indent}"))
}

/// The leading whitespace before `column` on the given source line.
fn line_indentation(source: &str, line: usize, column: usize) -> String {
    let line_text = source.split_inclusive('\n').nth(line).unwrap_or("");
    let prefix = line_text.get(..column).unwrap_or(line_text);

    if prefix.chars().all(char::is_whitespace) {
        prefix.to_owned()
    } else {
        String::new()
    }
}
