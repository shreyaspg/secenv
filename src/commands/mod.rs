pub mod init;
pub mod profile;


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
