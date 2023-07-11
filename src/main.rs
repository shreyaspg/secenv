pub mod application;
pub mod commands;
pub mod config;
pub mod prelude;

use crate::prelude::*;
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = SecEnv::parse();
    App::run(cli);
    ExitCode::from(0)
}
