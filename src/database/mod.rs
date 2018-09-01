use std::fs;
use std::fs::File;
extern crate sha1;
extern crate flate2;
use self::flate2::Compression;
use self::flate2::write::ZlibEncoder;
use constants::*;
use std::io::Write;
use std::collections::HashMap;

pub fn get_index_contents() -> HashMap<String, String> {
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


pub fn store_blob(filename: String) -> String {
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
    e.write_all(bytes).expect("could not write deflated contents");
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
    let mut obj_file = File::create(&sha1_filepath).expect("could not create sha1 file");
    obj_file.write_all(&bytes).expect("could not write deflated contents to sha1 file");
    sha1_filepath.to_string()
}