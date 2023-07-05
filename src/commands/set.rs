use clap::Args;

use super::Command;

#[derive(Args, Debug)]
pub struct Set {
    profile: Option<String>,
}

impl Command for Set{

}

