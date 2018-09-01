use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use constants::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::os::unix::fs::MetadataExt;

struct FileMeta {
    inode: String,
    last_mod_secs_from_epoch: String,
    filename: String
}

pub fn update_index(filename: &str) {
    let filemeta = get_file_metadata(&filename);
    let mut index_hash = get_index_contents();
    if new_or_updated_file(&filemeta, &index_hash) {
        update_in_memory_hash(&filemeta, &mut index_hash);
        let index_file = truncate_index_file();
        write_index_to_disk(index_file, &index_hash);
    }
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

fn get_index_contents() -> HashMap<String, String> {
    create_index_if_necessary();

    let index_contents = fs::read_to_string(INDEX_PATH);

    let lines = match index_contents {
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

fn create_index_if_necessary() {
    match File::open(INDEX_PATH) {
        Err(_) => match File::create(INDEX_PATH) {
            Err(e) => panic!("cannot create index: {:?}", e),
            Ok(_) => ()
        },
        Ok(_) => ()
    };
}

fn new_or_updated_file(filemeta: &FileMeta, hash: &HashMap<String, String>) -> bool {
    let inode = &filemeta.inode;
    !hash.contains_key(inode)
        || hash.get(inode).unwrap().to_string() != filemeta.last_mod_secs_from_epoch
}

fn update_in_memory_hash(filemeta: &FileMeta, index_hash: &mut HashMap<String, String>) {
    let inode = filemeta.inode.clone();
    let last_mod_date = filemeta.last_mod_secs_from_epoch.clone();
    index_hash.insert(inode, last_mod_date);
}

fn truncate_index_file() -> File {
    File::create(INDEX_PATH).expect("writing index: could not open index")
}

fn write_index_to_disk(mut index: File, hash: &HashMap<String, String>) {
    for key in hash.keys() {
        let new_index_entry = format!("{},{}\n", key, hash.get(key).unwrap());
        let bytes = new_index_entry.as_bytes();
        index.write_all(bytes).expect("writing index: could not write");
    }
}

// TODO: remove dependency on env::set_current_dir to parallelize tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

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

        update_index(&filemeta.filename);

        let mut index_hash_after = get_index_contents();
        assert_eq!(index_hash_after.get(&filemeta.inode),
                   Some(&filemeta.last_mod_secs_from_epoch.to_string()));

        env::set_current_dir("..");
        fs::remove_dir_all(test_dir);
    }
}
