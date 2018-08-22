use std::env;
use commands::Command;
use std::fs;
use std::fs::File;


pub struct AddCommand;

impl Command for AddCommand {
    fn execute(&self, args: Vec<String>) -> String {

        //////////////////////////////
        // implementation goes here //
        //////////////////////////////
        create_index_if_necessary();



        // create index (if it does not exist)
        // read index contents into hash table: inode => metadata struct
        // if no change => do nothing
        // else...
        //

        // feature scope: mgit add path/filename

        // check for file change
        //      if no change => do nothing
        //      else ...
        //          store blob
        //              concat header onto contents
        //              calculate sha1 of header+contents
        //              DEFLATE header+contents
        //              store deflated stuff at '.git/objects/' + sha1[0,2] + '/' + sha1[2,38]
        //          add entry to .git/index
        //              inode => metadata { last modified date, last changed date, sha1 }
        "Index updated".to_string()
    }
}

fn create_index_if_necessary() {
    let index_path = "./.mgit/index";
    match File::open(index_path) {
        Err(_) => match File::create(index_path) {
            Err(_) => panic!("cannot create index"),
            Ok(_) => ()
        } ,
        Ok(_) => ()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_index_when_not_present() {
        fs::create_dir("./.mgit");

        create_index_if_necessary();

        let index_path = "./.mgit/index";
        match File::open(index_path) {
            Err(_) => panic!("index does not exist"),
            Ok(_) => ()
        }

        fs::remove_dir_all(index_path);
    }
}