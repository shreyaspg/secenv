use clap::{Parser, Subcommand };

use crate::commands::Command;
use crate::commands::profile::Profile;
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
    Profile(Profile),
}

pub struct App;

impl App {
    pub fn run(cli : SecEnv) {
        if let Some(command) = cli.command {
            match command{
                Commands::Init(init_command) => {
                    init_command.execute();
                },
                Commands::Profile(profile) => {
                    profile.profile_subcommand
                        .expect("subcommand missing ")
                        .execute();
                },
            }
        } else {
            panic!("No command passed")
        }
    }
}
