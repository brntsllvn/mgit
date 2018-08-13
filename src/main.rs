fn main() {
    let default_output = vec!(
        "command        Description".to_string(),
        "----------------------------------------------".to_string(),
        "init           Create an empty Git repository".to_string(),
        "add            Add file contents to the index".to_string(),
        "commit         Record changes to the repository".to_string()
    );

    println!("{}", '\n');
    for item in default_output.iter() {
        println!("{}", item);
    }
    println!("{}", '\n');
}