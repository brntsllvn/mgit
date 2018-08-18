use std::env;

mod commands;
use commands::*;

fn main() {
    let mut args= env::args();
    let _filepath= args.next();
    let command= args.next();
    let result= match command {
        Some(cmd) => match cmd.as_ref() {
            "init" => commands::init::InitCommand.execute(args),
            "add" => commands::add::AddCommand.execute(args),
            "commit" => commands::commit::CommitCommand.execute(args),
            _ => commands::missing::MissingCommand.execute(args)
        },
        None => commands::empty::EmptyCommand.execute(args)
    };

    println!("\n{}\n", result);
}