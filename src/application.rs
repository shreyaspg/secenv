use clap::{Parser, Subcommand, Args};
// use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct SecEnv {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init(Init),
    Set(Set),
}

#[derive(Args)]
struct Init {
    path: Option<String>,
}

#[derive(Args,Debug)]
struct Set {
    profile: Option<String>,
}
pub struct App;

impl App {
    pub fn run(cli : SecEnv){
        println!("Starting app")
    }
}
