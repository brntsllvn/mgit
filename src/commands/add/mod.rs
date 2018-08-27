use commands::Command;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::env;
use constants::*;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::os::unix::fs::MetadataExt;
extern crate sha1;

extern crate flate2;
use std::io::prelude::*;
use commands::add::flate2::Compression;
use commands::add::flate2::write::ZlibEncoder;

pub struct AddCommand;

impl Command for AddCommand {
    fn execute(&self, args: Vec<String>) -> String {
        let filename = args.iter().next().expect("missing filename");
        let filemeta = get_file_metadata(filename);
        create_index_if_necessary();
        let mut index_hash = get_index_contents();
        if new_or_updated_file(&filemeta.inode, &filemeta.last_mod_secs_from_epoch, &index_hash) {
            process(&filemeta, &mut index_hash);
        }
        println!("\n{:?}\n", index_hash); // replace with `mgit status`
        "Index updated".to_string()
    }
}

fn process(filemeta: &FileMeta, index_hash: &mut HashMap<String, String>) {
    store_blob(filemeta.filename.clone());
    upsert_entry_into_index_hash(&filemeta, index_hash);
    write_hash_to_index(&index_hash);
}

fn store_blob(filename: String) -> String {
    let file = File::open(&filename).expect("storing blob: cannot open file");
    let file_contents = fs::read_to_string(&filename).expect("storing bloc: cannot read file contents");
    let header_plus_contents = concat_header_onto_contents(&file_contents);
    let sha1 = calculate_sha1(&header_plus_contents);
    let deflated_contents = deflate_contents(&header_plus_contents);
    let sha1_path = store_deflated_contents(&sha1, deflated_contents);
    sha1_path
}

fn concat_header_onto_contents(s: &str) -> String {
    format!("blob {}{}{}", s.len(), '\u{0000}', s)
}

fn calculate_sha1(s: &str) -> String {
    sha1::Sha1::from(s).digest().to_string()
}

fn deflate_contents(s: &str) -> Vec<u8> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    let contents = format!("{}", s);
    let bytes = contents.as_bytes();
    e.write_all(bytes);
    let compressed_bytes = e.finish();
    compressed_bytes.expect("could not deflate bytes")
}

fn store_deflated_contents(sha1: &str, bytes: Vec<u8>) -> String {
    let sha1_dir = format!("{}/{}", OBJ_PATH.to_owned(), &sha1[0..2]);
    match fs::read_dir(&sha1_dir) {
        Ok(_) => (),
        Err(_) => fs::create_dir(&sha1_dir).expect(&format!("could not create sha1 dir {}", &sha1_dir))
    }
    let sha1_filepath = format!("{}/{}", sha1_dir, &sha1[2..]);
    let mut obj_file = File::create(&sha1_filepath).expect("could not write deflated contents to sha1 file");
    obj_file.write_all(&bytes);
    sha1_filepath.to_string()
}

fn upsert_entry_into_index_hash(filemeta: &FileMeta, index_hash: &mut HashMap<String, String>) {
    let inode = filemeta.inode.clone();
    let last_mod_date = filemeta.last_mod_secs_from_epoch.clone();
    index_hash.insert(inode, last_mod_date);
}

fn write_hash_to_index(index_hash: &HashMap<String, String>) {
    let mut index = File::create(INDEX_PATH).expect("writing index: could not open index");
    for key in index_hash.keys() {
        let new_index_entry = format!("{},{}\n", key, index_hash.get(key).unwrap());
        let bytes = new_index_entry.as_bytes();
        index.write_all(bytes).expect("writing index: could not write");
        index.sync_data();
    }
}

fn nothing_to_do() {
    println!("unchanged");
}

struct FileMeta {
    inode: String,
    last_mod_secs_from_epoch: String,
    filename: String
}

fn new_or_updated_file(inode: &str, last_mod: &str, hash: &HashMap<String, String>) -> bool {
    !hash.contains_key(inode) || hash.get(inode).unwrap().to_string() != last_mod
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

        let sha1_path = store_blob(new_filepath.to_string());

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