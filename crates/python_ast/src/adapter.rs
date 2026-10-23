use qctidy_ports::{PythonAnalysis, PythonAstPort};

use crate::qiskit::analyze;

/// Parses Qiskit Python source into a circuit analysis.
#[derive(Debug, Default, Clone, Copy)]
pub struct PythonAstAdapter;

impl PythonAstPort for PythonAstAdapter {
    fn parse(&self, source: &str) -> PythonAnalysis {
        analyze(source)
    }
}
