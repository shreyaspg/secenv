pub mod init;
pub mod profile;

use toml::{Table, Value};
use std::fs::read_to_string;
use xdg::BaseDirectories;
use std::path::{ PathBuf, Path};

pub trait Command {
    fn execute(&self)->();
}

/// Get the config from defaul xdg path
fn get_config()-> PathBuf {
    let xdg_dirs = BaseDirectories::with_prefix("secenv").expect("Error finding the config dirs");
    let config_path = xdg_dirs
        .place_config_file("secenv.toml")
        .expect("cannot create configuration directory");
    config_path
}

/// Checks and returns config if exists, exit program 1 if not
pub fn does_config_exist() -> PathBuf{
    let config_path = get_config();
    if !Path::new(&config_path).exists(){
        eprintln!("Missing config file");
        std::process::exit(1)
    }
    config_path
}

fn get_profile_config() -> Table{
    let config_path = does_config_exist();
    let file = match read_to_string(config_path) {
        Ok(content) => content,
        Err(e) => panic!("{}",e)
    };
    let cfg: Table = file.parse().expect("Error getting config");
    if cfg.is_empty(){
        exit_with_error("No profiles found");
    }
    cfg
}

fn get_profile(config: Table, profile_name:&str) -> Option<Value>{
    let profile: Value;
    match &config["profiles"] {
        Value::Table(t) => {
            profile = t.get(profile_name).expect("Profile not found").clone();
            Some(profile)
        },
        _ => None
    }
}
/// exit with error
fn exit_with_error(message: &str){
    eprintln!("{}", message);
    std::process::exit(1)
}
