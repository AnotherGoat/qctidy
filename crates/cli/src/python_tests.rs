use crate::python::{JsonAnalysis, analyze};

#[test]
fn analysis_json_contains_the_parsed_circuits() {
    let source = "from qiskit import QuantumCircuit\n\nqc = QuantumCircuit(2)\nqc.h(0)\n";
    let analysis = analyze(source);
    let json = serde_json::to_value(JsonAnalysis::new("example.py", &analysis))
        .expect("the analysis should serialize");

    assert_eq!(json["filename"], "example.py");
    assert_eq!(json["module_circuits"][0]["name"], "qc");

    let gates = json["module_circuits"][0]["gates"]
        .as_array()
        .expect("gates should be an array");

    assert_eq!(gates.len(), 1);
}
