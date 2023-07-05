use clap::Args;

#[derive(Args, Debug)]
pub struct Init {
    path: Option<String>,
}
