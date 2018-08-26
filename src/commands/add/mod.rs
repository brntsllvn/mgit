use commands::Command;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::env;
use constants::*;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::os::unix::fs::MetadataExt;

pub struct AddCommand;

impl Command for AddCommand {
    fn execute(&self, args: Vec<String>) -> String {
        let filename = args.iter().next().expect("missing filename");
        let file_meta = get_file_metadata(filename);
        create_index_if_necessary();
        let index_hash = get_index_contents();
        match get_add_action(file_meta, index_hash) {
            FileStatus::Untracked => process_untracked_file(filename.to_string()),
            FileStatus::Changed => process_changed_file(filename.to_string()),
            FileStatus::Unchanged => nothing_to_do()
        }
        "Index updated".to_string()
    }
}

fn process_untracked_file(filename: String) {
    // store blob
    // add entry to index hash
    // clear index
    // write index from hash
    println!("untracked");
}

fn process_changed_file(filename: String) {
    // store blob
    // remove old entry from index
    // add entry to index hash
    // clear index
    // write index from hash
    println!("change");
}

fn nothing_to_do() {
    println!("unchanged");
}

// store blob
//      concat header onto contents
//      calculate sha1 of header+contents
//      DEFLATE header+contents
//      store deflated stuff at '.git/objects/' + sha1[0,2] + '/' + sha1[2,38]

struct FileMeta {
    inode: String,
    last_mod_secs_from_epoch: String
}

enum FileStatus {
    Untracked,
    Changed,
    Unchanged
}

fn get_add_action(file_meta: FileMeta, index_hash: HashMap<String, String>) -> FileStatus {
    if !index_hash.contains_key(&file_meta.inode) {
        FileStatus::Untracked
    } else {
        let cached_last_mod_data = index_hash.get(&file_meta.inode).unwrap().to_string();
        if cached_last_mod_data != file_meta.last_mod_secs_from_epoch {
            FileStatus::Changed
        } else {
            FileStatus::Unchanged
        }
    }
}

fn get_file_metadata(filename: &str) -> FileMeta {
    match fs::metadata(filename) {
        Ok(metadata) => FileMeta {
            inode: metadata.ino().to_string(),
            last_mod_secs_from_epoch: to_str(metadata.modified().unwrap())
        },
        Err(_) => panic!("cannot retrieve file metadata from {}", filename)
    }
}

fn to_str(last_mod_date: SystemTime) -> String {
    let since_the_epoch = last_mod_date
        .duration_since(UNIX_EPOCH)
        .expect("something went wrong");;
    let ms = since_the_epoch.as_secs() * 1000;
    ms.to_string()
}


fn create_index_if_necessary() {
    match File::open(INDEX_PATH) {
        Err(_) => match File::create(INDEX_PATH) {
            Err(e) => panic!("cannot create index: {:?}", e),
            Ok(_) => ()
        },
        Ok(_) => ()
    };
}

fn get_index_contents() -> HashMap<String, String> {
    let index_contents = fs::read_to_string(INDEX_PATH);

    let mut lines = match index_contents {
        Ok(ref file_contents) => file_contents.lines(),
        Err(_) => panic!("failed to split contents")
    };

    let mut map = HashMap::new();
    for line in lines {
        let key_val: Vec<&str> = line.split(",").collect();
        let inode = key_val.get(0).unwrap().to_string();
        let last_mod_date = key_val.get(1).unwrap().to_string();
        map.insert(inode,last_mod_date);
    }

    map
}


// TODO: remove dependency on env::set_current_dir to parallelize tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_filename_from_args() {
        get_file_metadata("./.gitignore");
//        assert_eq!(true, false);
    }

    #[test]
    fn create_index_when_not_present() {
        let test_dir = "./TEST_index_not_present";
        fs::create_dir(test_dir);
        env::set_current_dir(&test_dir).is_ok();

        fs::create_dir(MGIT_PATH);

        create_index_if_necessary();

        match File::open(INDEX_PATH) {
            Err(_) => panic!("index does not exist"),
            Ok(_) => ()
        }

        env::set_current_dir("..");
        fs::remove_dir_all(test_dir);
    }

    #[test]
    fn retrieve_empty_index_into_hashmap() {
        let test_dir = "./TEST_empty_index_hash";
        fs::create_dir(test_dir);
        env::set_current_dir(&test_dir).is_ok();

        fs::create_dir(MGIT_PATH);
        let mut file = match File::create(INDEX_PATH) {
            Ok(file) => file,
            Err(e) => panic!("{:?}", e)
        };

        let inode_to_meta = get_index_contents();

        assert_eq!(inode_to_meta.get("1"), None);

        env::set_current_dir("..");
        fs::remove_dir_all(test_dir);
    }

    #[test]
    fn retrieve_populated_index_into_hashmap() {
        let test_dir = "./TEST_index_into_hash";
        fs::create_dir(test_dir);
        env::set_current_dir(&test_dir).is_ok();

        fs::create_dir(MGIT_PATH);
        let mut file = match File::create(INDEX_PATH) {
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

        env::set_current_dir("..");
        fs::remove_dir_all(test_dir);
    }
}