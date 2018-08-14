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

// InitCommand.execute...returns "Initialized empty git repo"
// AddCommand.execute...returns ""
// CommitCommand.execute...returns "SHA-1"

fn main() {
    match env::args().skip(1).next() {
        Some(command) => match command.as_ref() {
            "init" => println!("running init..."),
            "add" => println!("running add..."),
            "commit" => println!("running commit..."),
            _ => println!("something went wrong: unrecognized command")
        },
        None => {
            EmptyCommand.execute();
            process::exit(0);
        }
    };
}