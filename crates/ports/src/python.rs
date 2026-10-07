use qctidy::{Circuit, GateOperation};

/// Parses Python source into a circuit analysis.
pub trait PythonPort {
    /// Parse a Python source file into the circuits and scopes it defines.
    fn parse(&self, source: &str) -> PythonAnalysis;
}

/// The circuits and scopes found in a Python source file.
#[derive(Debug, Clone)]
pub struct PythonAnalysis {
    /// The functions and classes found at the top level, in source order.
    pub scopes: Vec<Scope>,
    /// The circuits defined at module level, outside any function or class.
    pub module_circuits: Vec<ParsedCircuit>,
}

/// The kind of scope that groups circuits together.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeType {
    /// A function definition.
    Function,
    /// A class definition.
    Class,
}

/// A function or class that groups circuits together.
#[derive(Debug, Clone)]
pub struct Scope {
    /// Whether this scope is a function or a class.
    pub scope_type: ScopeType,
    /// The name of the function or class.
    pub name: String,
    /// The line where the scope starts.
    pub line: usize,
    /// The column where the scope starts.
    pub column: usize,
    /// The nested scopes and circuits, in source order.
    pub children: Vec<ScopeChild>,
}

/// A child of a scope: either a nested scope or a circuit.
#[derive(Debug, Clone)]
pub enum ScopeChild {
    /// A nested function or class.
    Scope(Scope),
    /// A circuit defined in this scope.
    Circuit(ParsedCircuit),
}

/// A circuit assignment found in the Python source.
#[derive(Debug, Clone)]
pub struct ParsedCircuit {
    /// The name of the variable that holds the circuit.
    pub name: String,
    /// The raw text of the qubit count argument, such as `4` or `NUM_QUBITS`.
    pub qubits: String,
    /// The raw text of the classical bit count argument.
    pub clbits: String,
    /// The line where the assignment starts.
    pub line: usize,
    /// The column where the assignment starts.
    pub column: usize,
    /// The gate calls applied to this circuit, in source order.
    pub gates: Vec<ParsedGate>,
    /// Every problem that prevents the circuit from being checked.
    pub issues: Vec<CircuitIssue>,
    /// The checkable circuit and its source map, when there are no issues.
    pub build: Option<CircuitBuild>,
}

/// A gate call applied to a circuit.
#[derive(Debug, Clone)]
pub struct ParsedGate {
    /// The name of the method that was called, such as `h`.
    pub gate_name: String,
    /// The name shown for this gate in the tree view.
    pub display_name: String,
    /// A human-readable description of the gate and its arguments.
    pub description: String,
    /// The line where the call starts.
    pub line: usize,
    /// The column where the call starts.
    pub column: usize,
    /// The line where the call ends.
    pub end_line: usize,
    /// The column where the call ends.
    pub end_column: usize,
    /// The arguments of the call, in order.
    pub parameters: Vec<GateParameter>,
    /// The canonical operation, when the gate can be checked.
    pub operation: Option<GateOperation>,
    /// Why the gate cannot be checked, when `operation` is `None`.
    pub reason: Option<String>,
}

/// A gate call argument and the source position it occupies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateParameter {
    /// The name of the argument, such as `qubit`.
    pub name: String,
    /// The raw text of the argument.
    pub value: String,
    /// The line where the argument starts.
    pub line: usize,
    /// The column where the argument starts.
    pub column: usize,
}

/// A problem that prevents a circuit from being checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitIssue {
    /// A human-readable description of the problem.
    pub message: String,
    /// The line where the problem was found.
    pub line: usize,
    /// The column where the problem was found.
    pub column: usize,
}

/// A checkable circuit and the source range of each operation.
#[derive(Debug, Clone)]
pub struct CircuitBuild {
    /// The circuit built from the parsed gates.
    pub circuit: Circuit,
    /// The source range of each operation, indexed like the circuit operations.
    pub source_map: Vec<SourceLocation>,
}

/// The source range occupied by a single operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLocation {
    /// The index of the operation in the circuit.
    pub index: usize,
    /// The line where the operation starts.
    pub line: usize,
    /// The column where the operation starts.
    pub column: usize,
    /// The line where the operation ends.
    pub end_line: usize,
    /// The column where the operation ends.
    pub end_column: usize,
}
