use commands::Command;
use database::index::get_index_contents;

pub struct StatusCommand;

impl Command for StatusCommand {
    fn execute(&self, _args: Vec<String>) -> String {
        let hash = get_index_contents();
        for line in hash.values() {
            println!("{},{},{},{},{}",
                line.mode,
                line.mgit_type,
                line.sha1,
                line.filename,
                line.last_mod
            )
        }

        "".to_string()
    }
}