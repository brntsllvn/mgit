use commands::Command;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use constants::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::os::unix::fs::MetadataExt;
use database::*;

pub struct AddCommand;

impl Command for AddCommand {
    fn execute(&self, args: Vec<String>) -> String {
        let filename = args.iter().next().expect("missing filename");
        let filemeta = get_file_metadata(filename);
        let mut index_hash = get_index_contents();
        if new_or_updated_file(&filemeta.inode, &filemeta.last_mod_secs_from_epoch, &index_hash) {
            process(&filemeta, &mut index_hash);
        }
        println!("\n{:?}\n", index_hash); // replace with `mgit status`
        "Index updated".to_string()
    }
}

fn process(filemeta: &FileMeta, index_hash: &mut HashMap<String, String>) {
    save_blob(filemeta.filename.clone());
    upsert_entry_into_index_hash(&filemeta, index_hash);
    save_index(&index_hash);
}

fn upsert_entry_into_index_hash(filemeta: &FileMeta, index_hash: &mut HashMap<String, String>) {
    let inode = filemeta.inode.clone();
    let last_mod_date = filemeta.last_mod_secs_from_epoch.clone();
    index_hash.insert(inode, last_mod_date);
}

struct FileMeta {
    inode: String,
    last_mod_secs_from_epoch: String,
    filename: String
}

fn get_file_metadata(filename: &str) -> FileMeta {
    match fs::metadata(filename) {
        Ok(metadata) => FileMeta {
            inode: metadata.ino().to_string(),
            last_mod_secs_from_epoch: to_str(metadata.modified().unwrap()),
            filename: filename.to_string()
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


// TODO: remove dependency on env::set_current_dir to parallelize tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

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

    #[test]
    fn store_a_new_blob() {
        let test_dir = "./TEST_store_a_new_blob";
        fs::create_dir(test_dir);
        env::set_current_dir(&test_dir).is_ok();
        fs::create_dir(MGIT_PATH);
        fs::create_dir(OBJ_PATH);

        let new_filepath = "./test.txt";
        let mut new_file = File::create(new_filepath).expect("could not create test file");
        let file_contents = "some file contents".as_bytes();
        new_file.write_all(file_contents);

        let sha1_path = save_blob(new_filepath.to_string());

        match File::open(sha1_path) {
            Err(_) => panic!("sha1 path does not exist"),
            Ok(_) => ()
        }

        env::set_current_dir("..");
        fs::remove_dir_all(test_dir);
    }

    #[test]
    fn update_index_for_untracked_file() {
        let test_dir = "./TEST_update_index_for_untracked";
        fs::create_dir(test_dir);
        env::set_current_dir(&test_dir).is_ok();
        fs::create_dir(MGIT_PATH);
        fs::create_dir(OBJ_PATH);
        File::create(INDEX_PATH);

        let new_filepath = "./test.txt";
        let new_file = File::create(new_filepath).expect("could not create test file");
        let filemeta = get_file_metadata(new_filepath);

        let mut index_hash_before = get_index_contents();

        process(&filemeta, &mut index_hash_before);

        let mut index_hash_after = get_index_contents();
        assert_eq!(index_hash_after.get(&filemeta.inode),
                   Some(&filemeta.last_mod_secs_from_epoch.to_string()));

        env::set_current_dir("..");
        fs::remove_dir_all(test_dir);
    }
}