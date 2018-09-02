use commands::Command;
use database::{save_blob, index::update_index};

pub struct AddCommand;

impl Command for AddCommand {
    fn execute(&self, args: Vec<String>) -> String {
        let filename = args.iter().next().expect("missing filename");
        let sha1 = save_blob(&filename);
        update_index(&filename, &sha1);
        "Index updated".to_string()
    }
}