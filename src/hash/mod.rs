extern crate sha1;

pub fn calculate_sha1(s: &str) -> String {
    sha1::Sha1::from(s).digest().to_string()
}