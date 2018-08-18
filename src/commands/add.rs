use std::env;
use commands::Command;

pub struct AddCommand;

impl Command for AddCommand {
    fn execute(&self, mut args: env::Args) -> String {

        //////////////////////////////
        // implementation goes here //
        //////////////////////////////

        "Index updated".to_string()
    }
}