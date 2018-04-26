extern crate serde_json;

use std::fs::{File, OpenOptions};
use std::fs;
use std::io::{Read, Write};
use std::io;
use std::path::Path;

pub fn manifest_reader(input: &str) -> Box<Read> {
    if input == "-" {
        Box::new(io::stdin())
    } else {
        let file = File::open(input).expect("Manifest file not found.");
        Box::new(file)
    }
}

pub fn manifest_writer(output: &str) -> Box<Write> {
    if output == "-" {
        Box::new(io::stdout())
    } else {
        if let Some(parent) = Path::new(output).parent() {
            fs::create_dir_all(parent).expect("Could not create output directory.");
        }

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(output)
            .expect("Unable to create manifest file.");
        Box::new(file)
    }
}
