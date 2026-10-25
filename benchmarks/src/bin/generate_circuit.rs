use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use qctidy::random;
use qctidy_codegen::CodegenAdapter;
use qctidy_ports::{CodeGenerationTarget, CodegenPort};

/// Defaults chosen so the generated circuit is large enough to profile the
/// hot paths but still checks in a few minutes.
const DEFAULT_SEED: u64 = 42;
const DEFAULT_QUBITS: usize = 50;
const DEFAULT_TIME_STEPS: usize = 1000;

fn main() -> ExitCode {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let mut seed = DEFAULT_SEED;
    let mut qubits = DEFAULT_QUBITS;
    let mut time_steps = DEFAULT_TIME_STEPS;
    let mut circuit_name = String::from("circuit");
    let mut output = None;

    let mut index = 0;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--seed" => {
                seed = value(&arguments, &mut index)
                    .parse()
                    .expect("--seed needs a number");
            }
            "--qubits" => {
                qubits = value(&arguments, &mut index)
                    .parse()
                    .expect("--qubits needs a number");
            }
            "--time-steps" => {
                time_steps = value(&arguments, &mut index)
                    .parse()
                    .expect("--time-steps needs a number");
            }
            "--circuit-name" => circuit_name = value(&arguments, &mut index).to_owned(),
            "-o" | "--output" => output = Some(PathBuf::from(value(&arguments, &mut index))),
            _ => {
                eprintln!("error: unknown argument '{}'", arguments[index]);
                return ExitCode::from(2);
            }
        }
        index += 1;
    }

    let graph = random::generate(seed, time_steps, qubits);
    let code = CodegenAdapter
        .generate(&graph, CodeGenerationTarget::Qiskit, Some(&circuit_name))
        .expect("a generated graph should always be renderable");

    match output {
        Some(path) => {
            if let Err(error) = fs::write(&path, code) {
                eprintln!("error: failed to write '{}': {error}", path.display());
                return ExitCode::from(2);
            }
        }
        None => {
            let mut stdout = io::stdout().lock();
            if stdout
                .write_all(code.as_bytes())
                .and_then(|()| stdout.flush())
                .is_err()
            {
                return ExitCode::from(2);
            }
        }
    }

    ExitCode::SUCCESS
}

/// The value following the current flag.
fn value<'a>(arguments: &'a [String], index: &mut usize) -> &'a str {
    *index += 1;
    arguments
        .get(*index)
        .map(String::as_str)
        .unwrap_or_default()
}
