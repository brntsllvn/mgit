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
        InitCommand.execute(vec!["dummy arg".to_string()]);

        let mgit_path = "./.mgit";
        let results = vec![
            File::open(mgit_path),
            File::open("./.mgit/objects"),
            File::open("./.mgit/refs")
        ];
        for result in results {
            match result {
                Err(e) => panic!("{:?} dir does not exist", e),
                Ok(_) => ()
            }
        }

        fs::remove_dir_all(mgit_path);
    }
}