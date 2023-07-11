use super::{get_config, Command};
use clap::Args;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Args, Debug)]
pub struct Init;

impl Command for Init {
    fn execute(&self) -> () {
        let config_path = get_config();
        let exists = Path::new(&config_path).exists();
        let mut config_file: File;
        if exists == false {
            config_file = File::create(&config_path).expect("Failed to create the config dir");
            let contents = include_str!("../config/secenv.template");
            config_file
                .write_all(contents.as_bytes())
                .expect("Failed to write to config file");
        } else {
            println!("Config already exists at {:?}", config_path)
        }
    }
}
