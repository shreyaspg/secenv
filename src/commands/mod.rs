pub mod init;
pub mod profile;

pub trait Command {
    fn execute(&self)->();
}
