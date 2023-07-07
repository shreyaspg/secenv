use clap::Args;
use std::fs::File;
use std::path::Path;
use super::{Command, get_config};

#[derive(Args, Debug)]
pub struct Init; 

impl Command for Init{
    fn execute(&self)->() {
            let config_path = get_config();
            let exists = Path::new(&config_path).exists();
            if exists == false {
                let _config_file = File::create(&config_path).expect("Failed to create the config dir");
                println!("Created the config at {:?}", config_path);
            } else {
                println!("Config exists at {:?}", config_path)
            }
    }
}
