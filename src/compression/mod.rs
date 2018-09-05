extern crate flate2;
use self::flate2::Compression;
use self::flate2::write::ZlibEncoder;
use self::flate2::read::ZlibDecoder;
use std::io::{Read, Write};

pub fn deflate_contents(s: &str) -> Vec<u8> {
    let mut encoded_file = ZlibEncoder::new(
        Vec::new(),
        Compression::default()
    );

    let contents = format!("{}", s);
    let bytes = contents.as_bytes();
    encoded_file
        .write_all(bytes)
        .expect("could not write deflated contents");
    let compressed_bytes = encoded_file
        .finish()
        .expect("could not deflate bytes");
    compressed_bytes
}

pub fn reflate_contents(byte_vec: &Vec<u8>) -> String {
    let mut z = ZlibDecoder::new(&byte_vec[..]);
    let mut buffer = String::new();
    let _result = z.read_to_string(&mut buffer);
    buffer.to_string()
}