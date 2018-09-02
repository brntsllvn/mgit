use commands::Command;
extern crate regex;
use self::regex::Regex;
use database::save_commit;

pub struct CommitCommand;

impl Command for CommitCommand {
    fn execute(&self, args: Vec<String>) -> String {
        let mut input_iterator = args.iter();
        let msg_flag = input_iterator.next().expect("missing message flag");
        if msg_flag != "-m" { panic!("expected message flag: -m") };
        let msg = input_iterator.next().expect("missing message");
        let re = Regex::new("^[[:alnum:]|[:blank:]]{1,50}$").unwrap();
        assert!(re.is_match(&msg));
        let commit_sha1 = save_commit(&msg);
        commit_sha1.to_string()
    }
}