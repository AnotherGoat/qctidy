use qctidy_ports::{PythonAnalysis, PythonPort};

use crate::qiskit::analyze;

/// Parses Qiskit Python source into a circuit analysis.
#[derive(Debug, Default, Clone, Copy)]
pub struct PythonAdapter;

impl PythonPort for PythonAdapter {
    fn parse(&self, source: &str) -> PythonAnalysis {
        analyze(source)
    }
}
