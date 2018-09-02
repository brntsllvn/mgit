use commands::Command;
use std::fs;
use std::fs::File;
use filepaths::*;
use std::io::Write;

pub struct InitCommand;

impl Command for InitCommand {
    fn execute(&self, _args: Vec<String>) -> String {
        fs::create_dir(MGIT_PATH).expect("could not create mgit path");
        fs::create_dir(OBJ_PATH).expect("could not create obj path");
        fs::create_dir(REF_PATH).expect("could not create ref path");

        fs::create_dir(REFHEAD_PATH).expect("could not create ref head file");
        File::create(MASTER_PATH).expect("could not create master path");

        let mut head_file = File::create(HEAD_PATH).expect("could not create HEAD file");
        head_file
            .write_all(String::from("ref: refs/heads/master").as_bytes())
            .expect("could not write master entry to HEAD");
        
        "Initialized empty mgit repo".to_string()
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
            fs::File::open(REF_PATH),
            fs::File::open(REFHEAD_PATH),
            fs::File::open(HEAD_PATH)
        ];
        for result in results {
            match result {
                Err(e) => panic!("{:?} dir does not exist", e),
                Ok(_) => ()
            }
        }

        let contents = fs::read_to_string(HEAD_PATH);
        let mut lines = match contents {
            Ok(ref file_contents) => file_contents.lines(),
            Err(e) => panic!("{:?} dir does not exist", e)
        };

        let symbolic_ref = lines.next().unwrap();
        assert_eq!(symbolic_ref, "ref: refs/heads/master".to_string());

        env::set_current_dir("..");
        fs::remove_dir_all(test_dir);
    }
}