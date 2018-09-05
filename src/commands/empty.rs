use commands::Command;

pub struct EmptyCommand;

impl Command for EmptyCommand {
    fn execute(&self, _args: Vec<String>) -> String {"\
        command        description\n\
        ----------------------------------------------\n\
        init           create an empty mgit repository\n\
        add            add file contents to the index\n\
        status         show the working tree status\n\
        commit         record changes to the repository\n\
        cat-file       show compressed file contents"
        .to_string()
    }
}