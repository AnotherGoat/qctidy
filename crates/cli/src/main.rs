mod arguments;
mod ast;
mod check;
mod convert;
mod display;
mod error;
mod input;
mod output;
mod present;
mod progress;
mod python;
mod rules;

use std::process::ExitCode;

use anstream::ColorChoice;
use clap::Parser;

use crate::arguments::{Cli, Command};

fn main() -> ExitCode {
    let cli = Cli::parse();
    let color = ColorChoice::from(cli.color);

    match cli.command {
        Command::Ast(arguments) => ast::run(&arguments, color),
        Command::Check(arguments) => check::run(&arguments, color),
        Command::Convert(arguments) => convert::run(&arguments, color),
        Command::Display(arguments) => display::run(&arguments, color),
        Command::Present(arguments) => present::run(&arguments, color),
        Command::Rules => rules::run(color),
    }
}
