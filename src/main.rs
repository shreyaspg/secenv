pub mod prelude;
pub mod application;
pub mod commands;

use std::process::ExitCode;
use clap::Parser;
use crate::prelude::*;

fn main()-> ExitCode {
    let cli = SecEnv::parse();
    App::run(cli);
    ExitCode::from(0)
}

