extern crate md5;
extern crate serde_json;
extern crate walkdir;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::fs;
use std::io::{Read, Write};
use std::io;
use std::path::Path;
use std::str;

use md5::Digest;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn dir_to_manifest(dir: &str) -> Result<HashMap<String, String>, Box<Error>> {
    let walker = WalkDir::new(dir).into_iter();
    let mut manifest = HashMap::new();

    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry?;
        let path = entry.path();

        let md = fs::metadata(path).expect("Could not read file metadata.");
        if md.is_dir() {
            continue;
        }

        let path_display = path.display();

        let mut file = File::open(path).expect("File not found.");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .expect("Unable to read file.");

        let hash_str = format!("{:x}", md5::Md5::digest(&contents));
        let ext = path.extension().expect("Missing file extension.");
        let ext_str = ext.to_os_string()
            .into_string()
            .expect("Cannot convert to string.");
        let hashed_name = [hash_str, ext_str].join(".");

        manifest.insert(path_display.to_string(), hashed_name);
    }

    Ok(manifest)
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let default_output = "manifest.json".to_string();

    let input_dir = args.iter().nth(1).expect("Missing input directory.");
    let output = args.iter().nth(2).unwrap_or(&default_output);

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

    let manifest = dir_to_manifest(&input_dir).expect("Could not build manifest.");
    serde_json::to_writer(writer, &manifest).expect("Failed to serialize manifest.");
}
