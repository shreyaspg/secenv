use clap::{Parser, Subcommand, Args};

use crate::commands::set::Set;
use crate::commands::init::Init;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct SecEnv {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init(Init),
    Set(Set),
}

pub struct App;

impl App {
    pub fn run(cli : SecEnv) {
        if let Some(command) = cli.command {
                match command{
                    Commands::Init(path) => {
                        println!("Running init command {:?}", path )
                    },
                    Commands::Set(profile) => {
                        println!("Running set command {:?}", profile )
                    },
                }
        } else {
                panic!("No command passed")
        }
    }
}
