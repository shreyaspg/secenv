use std::fs::read_to_string;
use clap::{Args, Subcommand};
use toml::Table;
use super::{Command, does_config_exist};

#[derive(Args, Debug)]
pub struct Profile {
    #[command(subcommand)]
    pub profile_subcommand: Option<ProfileSubCommand>,
}

#[derive(Subcommand, Debug)]
pub enum ProfileSubCommand {
    List,
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
        match self {
            Self::List => {
                list_command();
            }
            Self::Set(set_args) => {
                set_command(set_args);
            }
            Self::Unset(_) => {
                unset_command();
            }
        }
    }
}

fn list_command() {
    let config_path = does_config_exist();
    let file = match read_to_string(config_path) {
        Ok(content) => content,
        Err(e) => panic!("{}",e)
    };
    let cfg: Table = file.parse().expect("Error getting config");
    let cfg_profiles = &cfg["profiles"];
    if let toml::Value::Table(t) = cfg_profiles {
        for (key, _) in t {
            println!("{key}");
        }
    }
}

fn set_command(args: &SetArgs) {
    let config_path = does_config_exist();
    let file = match read_to_string(config_path) {
        Ok(content) => content,
        Err(e) => panic!("{}",e)
    };

    let cfg: Table = file.parse().expect("Error getting config");
    let profile = args.name.clone().expect("No profile name found");
    println!("setting{}", cfg["profiles"][profile]);
}
fn unset_command() {
    let _config_path = does_config_exist();
}


