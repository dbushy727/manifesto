extern crate serde_json;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::fs;
use std::io::{Read, Write};
use std::io;
use std::path::Path;

fn read_manifest(manifest: &str) -> Result<HashMap<String, String>, Box<Error>> {
    let reader: Box<Read> = if manifest == "-" {
        Box::new(io::stdin())
    } else {
        let file = File::open(manifest).expect("Manifest file not found.");
        Box::new(file)
    };

    Ok(serde_json::from_reader(reader)?)
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let default_output = "manifest.json".to_string();

    let manifest_a = args.iter().nth(1).expect("Missing first manifest.json");
    let manifest_b = args.iter().nth(2).expect("Missing second manifest.json");
    let output = args.iter().nth(3).unwrap_or(&default_output);

    let mut manifest_a = read_manifest(manifest_a).expect("Could not load first manifest.json");
    let manifest_b = read_manifest(manifest_b).expect("Could not load second manifest.json");

    manifest_a.extend(manifest_b);

    let writer: Box<Write> = if output == "-" {
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
    };

    serde_json::to_writer(writer, &manifest_a).expect("Failed to serialize manifest.");
}
