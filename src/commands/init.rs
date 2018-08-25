use commands::Command;
use std::fs;
use std::fs::File;
use std::error::Error;
use constants::*;

pub struct InitCommand;

impl Command for InitCommand {
    fn execute(&self, args: Vec<String>) -> String {
        fs::create_dir(MGIT_PATH);
        fs::create_dir(OBJ_PATH);
        fs::create_dir(REF_PATH);
        "Initialized empty git repo".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_mgit_db() {
        InitCommand.execute(vec!["dummy arg".to_string()]);

        let results = vec![
            File::open(MGIT_PATH),
            File::open(OBJ_PATH),
            File::open(REF_PATH)
        ];
        for result in results {
            match result {
                Err(e) => panic!("{:?} dir does not exist", e),
                Ok(_) => ()
            }
        }

        fs::remove_dir_all(MGIT_PATH);
    }
}