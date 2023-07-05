use clap::Args;
use xdg::BaseDirectories;
use std::fs::File;
use std::path::Path;
use super::Command;

#[derive(Args, Debug)]
pub struct Init; 

impl Command for Init{
    fn execute(&self)->() {
            let xdg_dirs = BaseDirectories::with_prefix("secenv").expect("Error finding the config dirs");
            let config_path = xdg_dirs
                .place_config_file("secenv.toml")
                .expect("cannot create configuration directory");
            let exists = Path::new(&config_path).exists();
            if exists == false {
                let _config_file = File::create(&config_path).expect("Failed to create the config dir");
                println!("Created the config at {:?}", config_path);
            } else {
                println!("Config exists at {:?}", config_path)
            }
    }
}
