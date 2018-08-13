use std::env;
use std::process;


fn main() {
    let args = env::args();

    match args.skip(1).next() {
        Some(command) => match command.as_ref() {
            "init" => println!("running init..."),
            "add" => println!("running add..."),
            "commit" => println!("running commit..."),
            _ => println!("something went wrong: unrecognized command")
        },
        None => {
            println!("printing default stuff");
            process::exit(0);
        }
    };
}


//    let default_output = vec!(
//        "command        Description".to_string(),
//        "----------------------------------------------".to_string(),
//        "init           Create an empty Git repository".to_string(),
//        "add            Add file contents to the index".to_string(),
//        "commit         Record changes to the repository".to_string()
//    );




// accept command line args
// determine which struct (default, init, add, commit) to use (ideally make the return type an abstraction)
// create the struct
