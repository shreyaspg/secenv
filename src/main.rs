mod application;
use clap::Parser;
use crate::application::{SecEnv, App};

fn main() {
    let cli = SecEnv::parse();
    App::run(cli)
}

