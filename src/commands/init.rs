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
        let mgit_path = "./.mgit";
        let obj_path = "./.mgit/objects";
        let ref_path = "./.mgit/refs";
        let dummy_args = vec!["hi".to_string()];

        InitCommand.execute(dummy_args);

        let results = vec![
            File::open(mgit_path),
            File::open(obj_path),
            File::open(ref_path)
        ];
        for result in results {
            match result {
                Err(_) => panic!("dir does not exist"),
                Ok(_) => ()
            }
        }

        fs::remove_dir_all(mgit_path);
    }
}