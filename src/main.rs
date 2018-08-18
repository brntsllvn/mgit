use std::env;

mod commands;
use commands::*;

fn main() {
    let mut args= env::args();
    let _filepath= args.next();
    let command = commands::get_command(args.next());
    let result = command.execute(args);
    println!("\n{}\n", result);
}
