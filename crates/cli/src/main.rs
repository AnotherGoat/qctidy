#![allow(clippy::exit, clippy::print_stderr, clippy::print_stdout)]

mod analyze;
mod arguments;
mod check;
mod convert;
mod display;
mod error;
mod fix;
mod input;
mod output;
mod present;
mod progress;
mod python;
mod rules;

#[cfg(test)]
mod check_tests;
#[cfg(test)]
mod fix_tests;
#[cfg(test)]
mod input_tests;
#[cfg(test)]
mod python_tests;

use std::process::ExitCode;

use anstream::ColorChoice;
use clap::Parser;

use crate::arguments::{Cli, Command};

fn main() -> ExitCode {
    let cli = Cli::parse();
    let color = ColorChoice::from(cli.color());

    match cli.get_command() {
        Command::Analyze(arguments) => analyze::run(arguments, color),
        Command::Check(arguments) => check::run(arguments, color),
        Command::Convert(arguments) => convert::run(arguments, color),
        Command::Display(arguments) => display::run(arguments, color),
        Command::Present(arguments) => present::run(arguments, color),
        Command::Rules => rules::run(color),
    }
}
