use commands::Command;
use database::{blob::save_blob, index::update_index};

pub struct AddCommand;

impl Command for AddCommand {
    fn execute(&self, args: Vec<String>) -> String {
        let filename = args.iter().next().expect("missing filename");
        save_blob(&filename);
        update_index(&filename);
        "Index updated".to_string()
    }
}