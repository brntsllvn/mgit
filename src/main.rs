use std::env;
mod commands;
mod filepaths;
mod database;
mod hash;

fn main() {
    let mut args= env::args();
    let _filepath= args.next();
    let command = commands::get_command(args.next());
    let result = command.execute(args.collect());
    println!("\n{}\n", result);
}
