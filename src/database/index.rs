use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use filepaths::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::os::unix::fs::MetadataExt;

struct FileMeta {
    inode: String,
    last_mod: String,
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
            last_mod: to_str(metadata.modified().unwrap()),
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

struct IndexLine {
    inode: String,
    mode: String,
    mgit_type: String,
    sha1: String,
    filename: String,
    last_mod: String
}

fn get_index_contents() -> HashMap<String, IndexLine> {
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
        let index_line = IndexLine {
            inode: key_val.get(0).unwrap().to_string(),
            mode: key_val.get(1).unwrap().to_string(),
            mgit_type: key_val.get(2).unwrap().to_string(),
            sha1: key_val.get(3).unwrap().to_string(),
            filename: key_val.get(4).unwrap().to_string(),
            last_mod: key_val.get(5).unwrap().to_string()
        };
        map.insert(inode,index_line);
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

fn new_or_updated_file(filemeta: &FileMeta, hash: &HashMap<String, IndexLine>) -> bool {
    let inode = &filemeta.inode;
    !hash.contains_key(inode)
        || hash.get(inode).unwrap().last_mod != filemeta.last_mod
}

// TODO: pass in sha1
fn update_in_memory_hash(filemeta: &FileMeta, index_hash: &mut HashMap<String, IndexLine>) {
    let inode = filemeta.inode.clone();
    let last_mod_date = filemeta.last_mod.clone();
    let index_line = IndexLine {
        inode: filemeta.inode.clone(),
        mode: String::from("100644"),
        mgit_type: String::from("blob"),
        sha1: String::from ("345345345345345345"),
        filename: filemeta.filename.clone(),
        last_mod: filemeta.last_mod.clone()
    };
    index_hash.insert(inode, index_line);
}

fn truncate_index_file() -> File {
    File::create(INDEX_PATH).expect("writing index: could not open index")
}

fn write_index_to_disk(mut index: File, hash: &HashMap<String, IndexLine>) {

    for inode in hash.keys() {
        let index_line = hash.get(inode).unwrap();
        let new_index_entry = format!("{},{},{},{},{},{}\n",
                                      inode,
                                      index_line.mode,
                                      index_line.mgit_type,
                                      index_line.sha1,
                                      index_line.filename,
                                      index_line.last_mod,
        );
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

        match file.write_all(b"1,100644,blob,df4522,hi.txt,123\n\
                               2,040000,tree,da332f,some_dir,222")
        {
            Err(_) => panic!("something else went wrong"),
            Ok(_) => ()
        };

        let index_hash = get_index_contents();

        let index_line1 =  index_hash.get("1").unwrap();
        assert_eq!(index_line1.sha1, "df4522".to_string());

        let index_line2 =  index_hash.get("2").unwrap();
        assert_eq!(index_line2.filename, "some_dir".to_string());

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

        let mut index_hash = get_index_contents();

        let index_line1 =  index_hash.get(&filemeta.inode).unwrap();
        assert_eq!(index_line1.last_mod, filemeta.last_mod.to_string());

        env::set_current_dir("..");
        fs::remove_dir_all(test_dir);
    }
}
