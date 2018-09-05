use commands::Command;
use database::print_commit_history;

pub struct LogCommand;

impl Command for LogCommand {
    fn execute(&self, _args: Vec<String>) -> String {
        print_commit_history();
        "".to_string()
    }
}