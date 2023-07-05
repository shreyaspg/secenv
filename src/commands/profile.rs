use clap::{Args, Subcommand};

use super::Command;

#[derive(Args, Debug)]
pub struct Profile {
    #[command(subcommand)]
    profile_command: Option<ProfileSubCommand>,

    name: Option<String>,
    
}

#[derive(Subcommand, Debug)]
enum ProfileSubCommand {
    Set(SetArgs),

    Unset(UnsetArgs),
}

#[derive(Args, Debug)]
struct SetArgs{
    name: Option<String>,
}

#[derive(Args, Debug)]
struct UnsetArgs{
    name: Option<String>,
}

impl Command for Profile{
    fn execute(&self)->() {
        // let mut profile_name = self.name.as_ref().expect("No profile name passed");
        // if let Some(name) = &self.name{
        //     profile_name = name;
        // }
        println!("Setting profile: {:?}", self)
    }
}

