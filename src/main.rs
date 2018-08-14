use std::env;
use std::process;

pub trait Command {
    fn execute(&self) -> String;
}

pub struct EmptyCommand;

impl Command for EmptyCommand {
    fn execute(&self) -> String {
        let default = "
command        Description
----------------------------------------------
init           Create an empty Git repository
add            Add file contents to the index
commit         Record changes to the repository
";
        println!("{}", default);
        default.to_string()
    }
}

pub struct InitCommand;
impl Command for InitCommand {
    fn execute(&self) -> String {
        // implementation goes here
        let msg = "Initialized empty git repo".to_string();
        println!("{}", msg);
        msg
    }
}

pub struct AddCommand;
impl Command for AddCommand {
    fn execute(&self) -> String {
        // implementation goes here
        let msg = "Index updated".to_string();
        println!("{}", msg);
        msg
    }
}

pub struct CommitCommand;
impl Command for CommitCommand {
    fn execute(&self) -> String {
        // implementation goes here
        let msg = "Committed...".to_string();
        println!("{}", msg);
        msg // SHA-1
    }
}

fn main() {
    match env::args().skip(1).next() {
        Some(command) => match command.as_ref() {
            "init" => InitCommand.execute(),
            "add" => AddCommand.execute(),
            "commit" => CommitCommand.execute(),
            _ => {
                println!("mgit: '{}' is not a git command. See 'mgit' for help", command);
                process::exit(-1);
            }
        },
        None => {
            EmptyCommand.execute();
            process::exit(0);
        }
    };
}