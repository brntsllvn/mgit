use commands::Command;
use std::fs;
use filepaths::*;

pub struct InitCommand;

impl Command for InitCommand {
    fn execute(&self, _args: Vec<String>) -> String {
        fs::create_dir(MGIT_PATH).expect("could not create mgit path");
        fs::create_dir(OBJ_PATH).expect("could not create obj path");
        fs::create_dir(REF_PATH).expect("could not create ref path");
        "Initialized empty git repo".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn creates_mgit_db() {
        let test_dir = "./TEST_create_mgit_db";
        fs::create_dir(test_dir);
        env::set_current_dir(&test_dir).is_ok();

        InitCommand.execute(vec!["dummy arg".to_string()]);

        let results = vec![
            fs::File::open(MGIT_PATH),
            fs::File::open(OBJ_PATH),
            fs::File::open(REF_PATH)
        ];
        for result in results {
            match result {
                Err(e) => panic!("{:?} dir does not exist", e),
                Ok(_) => ()
            }
        }

        env::set_current_dir("..");
        fs::remove_dir_all(test_dir);
    }
}