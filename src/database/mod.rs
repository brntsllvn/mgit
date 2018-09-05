pub mod index;
use std::fs;
use std::fs::File;
extern crate sha1;
extern crate flate2;
use self::flate2::Compression;
use self::flate2::write::ZlibEncoder;
use self::flate2::read::ZlibDecoder;
use filepaths::*;
use std::io::{Read, Write};
use database::index::{
    get_index_contents,
    truncate_index_file,
    index_is_empty,
    IndexLine};
use std::collections::HashMap;


pub fn save_blob(filename: &str) -> String {
    let file_contents = fs::read_to_string(&filename).expect("storing bloc: cannot read file contents");
    let header_plus_contents = concat_header_onto_contents(&file_contents);
    let sha1 = calculate_sha1(&header_plus_contents);
    let deflated_contents = deflate_contents(&header_plus_contents);
    store_deflated_contents(&sha1, deflated_contents);
    sha1
}

pub fn save_commit(msg: &str) -> String {
    if index_is_empty() {
        return "".to_string()
    }

    let tree_sha1 = save_tree();
    let parent_sha1 = get_parent_sha1();
    let contents = format!("\
        tree {}\n\
        parent {}\n\
        \n\
        {}", tree_sha1, parent_sha1, msg);
    let sha1 = calculate_sha1(&contents);
    let deflated_contents = deflate_contents(&contents);
    store_deflated_contents(&sha1, deflated_contents);
    save_sha1_in_branch_file(&sha1);
    truncate_index_file();
    sha1
}

fn save_tree() -> String {
    let index_hash = get_index_contents();
    let contents = flatten_index_hash(&index_hash);
    let sha1 = calculate_sha1(&contents);
    let deflated_contents = deflate_contents(&contents);
    store_deflated_contents(&sha1, deflated_contents);
    sha1
}

fn get_parent_sha1() -> String {
    let index_contents = fs::read_to_string(MASTER_PATH);
    let mut lines = match index_contents {
        Ok(ref file_contents) => file_contents.lines(),
        Err(_) => panic!("failed to retrieve sha1 from master file")
    };

    let parent_sha1 = match lines.next() {
        Some(sha1) => sha1,
        None => "<none>"
    };
    parent_sha1.to_string()
}

fn save_sha1_in_branch_file(sha1: &str) {
    let mut master_file = File::create(MASTER_PATH).expect("could not open master file");
    master_file
        .write_all(sha1.as_bytes())
        .expect("could not update master file");
}

fn flatten_index_hash(index_hash: &HashMap<String, IndexLine>) -> String {
    let mut res = String::new();
    for index_line in index_hash.values() {
        res = res + &format!("{} {} {} {}\n",
            index_line.mode,
            index_line.mgit_type,
            index_line.sha1,
            index_line.filename);
    }
    res
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
    e.write_all(bytes).expect("could not write deflated contents");
    let compressed_bytes = e.finish();
    compressed_bytes.expect("could not deflate bytes")
}

fn store_deflated_contents(sha1: &str, bytes: Vec<u8>) {
    let sha1_dir = format!("{}/{}", OBJ_PATH.to_owned(), &sha1[0..2]);
    match fs::read_dir(&sha1_dir) {
        Ok(_) => (),
        Err(_) => fs::create_dir(&sha1_dir).expect(&format!("could not create sha1 dir"))
    }
    let sha1_filepath = format!("{}/{}", sha1_dir, &sha1[2..]);
    let mut obj_file = File::create(&sha1_filepath).expect("could not create sha1 file");
    obj_file.write_all(&bytes).expect("could not write deflated contents to sha1 file");
}

pub fn get_reflated_contents(sha1: &str) -> String {
    let sha1_path = format!("{}/{}/{}", OBJ_PATH.to_owned(), &sha1[0..2], &sha1[2..]);
    let byte_vec = fs::read(&sha1_path).expect("could not open sha1 file");
    let mut z = ZlibDecoder::new(&byte_vec[..]);
    let mut s = String::new();
    let _result = z.read_to_string(&mut s);
    s.to_string()
}

#[cfg(test)]
mod blob_test {
    use super::*;
    use std::env;

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

        let sha1 = save_blob(new_filepath.as_ref());

        let sha1_path = format!("{}/{}/{}-blob",
                                OBJ_PATH.to_owned(),
                                &sha1[0..2],
                                &sha1[2..]
        );
        match File::open(sha1_path) {
            Err(_) => panic!("sha1 path does not exist"),
            Ok(_) => ()
        }

        env::set_current_dir("..");
        fs::remove_dir_all(test_dir);
    }
}