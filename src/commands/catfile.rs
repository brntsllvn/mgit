use commands::Command;
use database::get_reflated_contents;

pub struct CatFileCommand;

impl Command for CatFileCommand {
    fn execute(&self, args: Vec<String>) -> String {
        let mut input_iterator = args.iter();
        let _content_flag = input_iterator.next().expect("missing content flag");
        let sha1 = input_iterator.next().expect("missing object sha1");
        let reflated_contents = get_reflated_contents(&sha1);
        println!("{}", reflated_contents);
        "".to_string()
    }
}