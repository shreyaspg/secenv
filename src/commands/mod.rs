pub mod init;
pub mod set;


pub trait Command {
    fn execute(){}
    fn help(){}
}
