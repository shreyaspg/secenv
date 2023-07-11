use super::{ get_profile_config, Command, get_profile};
use clap::{Args, Subcommand};

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
pub struct SetArgs {
    name: Option<String>,
}

#[derive(Args, Debug)]
pub struct UnsetArgs {
    name: Option<String>,
}
impl Command for ProfileSubCommand {
    fn execute(&self) -> () {
        match self {
            Self::List => {
                list_command();
            }
            Self::Set(set_args) => {
                set_command(set_args);
            }
            Self::Unset(unset_args) => {
                unset_command(unset_args);
            }
        }
    }
}

fn list_command() {
    let cfg = get_profile_config();
    let cfg_profiles = &cfg["profiles"];
    if let toml::Value::Table(t) = cfg_profiles {
        for (key, _) in t {
            println!("{key}");
        }
    }
}

fn set_command(args: &SetArgs) {
    let cfg = get_profile_config();
    let profile_name = args.name.clone().expect("No profile name found");
    let profile = get_profile(cfg, &*profile_name).unwrap();
    println!("setting{}", profile);
}
fn unset_command(args: &UnsetArgs) {
    let cfg = get_profile_config();
    let profile_name = args.name.clone().expect("No profile name found");
    let profile = get_profile(cfg, &*profile_name).unwrap();
    println!("setting{}", profile);
}
