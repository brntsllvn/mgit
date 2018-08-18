use std::env;

mod commands;
use commands::*;

fn main() {
    let mut args = env::args();
    let _filepath = args.next();
    let command = args.next();
    let result = match command {
        Some(cmd) => match cmd.as_ref() {
            "init" => commands::InitCommand.execute(args),
            "add" => commands::AddCommand.execute(args),
            "commit" => commands::CommitCommand.execute(args),
            _ => commands::MissingCommand.execute(args)
        },
        None => commands::empty::EmptyCommand.execute(args)
    };
    println!("{}", result);
}