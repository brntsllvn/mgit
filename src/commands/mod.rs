use std::env;
pub mod empty;
pub mod init;
pub mod add;
pub mod commit;
pub mod missing;

pub trait Command {
    fn execute(&self, mut args: env::Args) -> String;
}
