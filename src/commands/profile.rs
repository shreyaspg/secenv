use clap::{Args, Subcommand};

use super::Command;

#[derive(Args, Debug)]
pub struct Profile {
    #[command(subcommand)]
    pub profile_sub_command: Option<ProfileSubCommand>,
}

#[derive(Subcommand, Debug)]
pub enum ProfileSubCommand {
    Set(SetArgs),

    Unset(UnsetArgs),
}

#[derive(Args, Debug)]
pub struct SetArgs{
    name: Option<String>,
}

#[derive(Args, Debug)]
pub struct UnsetArgs{
    name: Option<String>,
}

impl Command for ProfileSubCommand{
    fn execute(&self)->() {
        // let mut profile_name = self.name.as_ref().expect("No profile name passed");
        // if let Some(name) = &self.name{
        //     profile_name = name;
        // }
        println!("Setting profile: {:?}", self)
    }
}

