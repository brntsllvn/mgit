use std::env;
use std::process;

mod commands;
use commands::Command;

fn main() {
    match env::args().skip(1).next() {
        Some(command) => match command.as_ref() {
            "init" => commands::InitCommand.execute(),
            "add" => commands::AddCommand.execute(),
            "commit" => commands::CommitCommand.execute(),
            _ => {
                commands::MissingCommand.execute();
                process::exit(-1);
            }
        },
        None => {
            commands::EmptyCommand.execute();
            process::exit(0);
        }
    };
}