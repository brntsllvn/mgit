use commands::Command;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use constants::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::os::unix::fs::MetadataExt;
use database;
use database::blob::save_blob;
use database::index::update_index;

pub struct AddCommand;

impl Command for AddCommand {
    fn execute(&self, args: Vec<String>) -> String {
        let filename = args.iter().next().expect("missing filename");
        save_blob(&filename);
        update_index(&filename);
        "Index updated".to_string()
    }
}