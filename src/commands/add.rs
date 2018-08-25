use std::env;
use commands::Command;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

pub struct AddCommand;

impl Command for AddCommand {
    fn execute(&self, args: Vec<String>) -> String {

        //////////////////////////////
        // implementation goes here //
        //////////////////////////////
        create_index_if_necessary();

        let inode_to_meta = get_index_contents();

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
            Err(e) => panic!("cannot create index: {:?}", e),
            Ok(_) => ()
        },
        Ok(_) => ()
    };
}

fn get_index_contents() -> HashMap<String, String> {
    let index_path = "./.mgit/index";
    let index_contents = fs::read_to_string(index_path);

    let mut lines = match index_contents {
        Ok(ref file_contents) => file_contents.lines(),
        Err(_) => panic!("failed to split contents")
    };

    let mut map = HashMap::new();
    for line in lines {
        let key_val: Vec<&str> = line.split(",").collect();
        map.insert(key_val.get(0).unwrap().to_string(),
                   key_val.get(1).unwrap().to_string());
    }

    println!("{:?}", map);
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_index_when_not_present() {
        let mgit_path = "./.mgit";
        fs::create_dir(mgit_path);

        create_index_if_necessary();

        let index_path = "./.mgit/index";
        match File::open(index_path) {
            Err(_) => panic!("index does not exist"),
            Ok(_) => ()
        }

        fs::remove_dir_all(mgit_path);
    }

    #[test]
    fn retrieve_empty_index_into_hashmap() {
        let mgit_path = "./.mgit";
        fs::create_dir(mgit_path);
        let mut file = match File::create("./.mgit/index") {
            Ok(file) => file,
            Err(e) => panic!("{:?}", e)
        };

        let inode_to_meta = get_index_contents();

        assert_eq!(inode_to_meta.get("1"), None);

        fs::remove_dir_all(mgit_path);
    }

    #[test]
    fn retrieve_populated_index_into_hashmap() {
        let mgit_path = "./.mgit";
        fs::create_dir(mgit_path);
        let mut file = match File::create("./.mgit/index") {
            Ok(file) => file,
            Err(e) => panic!("{:?}", e)
        };
        match file.write_all(b"1,123\n2,222") {
            Err(_) => panic!("something else went wrong"),
            Ok(_) => ()
        };

        let inode_to_meta = get_index_contents();

        assert_eq!(inode_to_meta.get("1"), Some(&"123".to_string()));
        assert_eq!(inode_to_meta.get("2"), Some(&"222".to_string()));

        fs::remove_dir_all(mgit_path);
    }
}