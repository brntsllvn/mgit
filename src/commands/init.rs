use std::env;
use commands::Command;
use std::fs;
use std::fs::File;
use std::error::Error;

pub struct InitCommand;

impl Command for InitCommand {
    fn execute(&self, args: Vec<String>) -> String {

        fs::create_dir("./.mgit");
        fs::create_dir("./.mgit/objects");
        fs::create_dir("./.mgit/refs");

        "Initialized empty git repo".to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_mgit_db() {
        let dummy_args = vec!["hi".to_string()];
        let path = "./.mgit";
        let command = InitCommand.execute(dummy_args);
        let result = File::open(path);
        match result {
            Err(_) => panic!("dir does not exist"),
            Ok(_) => println!("pass")
        };
        fs::remove_dir(path);
    }

    #[test]
    fn creates_obj_dir() {
        let dummy_args = vec!["hi".to_string()];
        let path = "./.mgit/objects";
        let command = InitCommand.execute(dummy_args);
        let result = File::open(path);
        match result {
            Err(_) => panic!("dir does not exist"),
            Ok(_) => println!("pass")
        };
        fs::remove_dir(path);
    }

    #[test]
    fn creates_refs_dir() {
        let dummy_args = vec!["hi".to_string()];
        let path = "./.mgit/refs";
        let command = InitCommand.execute(dummy_args);
        let result = File::open(path);
        match result {
            Err(_) => panic!("dir does not exist"),
            Ok(_) => println!("pass")
        };
        fs::remove_dir(path);
    }
}