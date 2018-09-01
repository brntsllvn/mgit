use commands::Command;

pub struct EmptyCommand;

impl Command for EmptyCommand {
    fn execute(&self, _args: Vec<String>) -> String {"\
        command        description\n\
        ----------------------------------------------\n\
        init           create an empty mgit repository\n\
        add            add file contents to the index\n\
        commit         record changes to the repository"
        .to_string()
    }
}