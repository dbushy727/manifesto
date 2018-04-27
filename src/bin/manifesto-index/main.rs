extern crate manifesto as m;
extern crate md5;
extern crate serde_json;
extern crate walkdir;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::fs;
use std::io::Read;
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

fn is_dir(path: &Path) -> bool {
    fs::metadata(path)
        .expect("Could not read file metadata.")
        .is_dir()
}

fn read_path(path: &Path) -> Vec<u8> {
    let mut file = File::open(path).expect("File not found.");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Unable to read file.");

    buf
}

fn manifest_from_dir(dir: &str) -> HashMap<String, String> {
    let walker = WalkDir::new(dir).into_iter();
    let mut manifest = HashMap::new();

    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.expect("No entries found.");
        let path = entry.path();

        if is_dir(path) {
            continue;
        }

        let buf = read_path(path);

        let hash = format!("{:x}", md5::Md5::digest(&buf));
        let ext = path.extension().expect("Missing file extension.");
        let ext = ext.to_os_string()
            .into_string()
            .expect("Cannot convert extention to string.");
        let hashed_name = [hash, ext].join(".");

        manifest.insert(path.display().to_string(), hashed_name);
    }

    manifest
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let default_output = "manifest.json".to_string();

    let input_dir = args.iter().nth(1).expect("Missing input directory.");
    let output = args.iter().nth(2).unwrap_or(&default_output);

    let manifest = manifest_from_dir(&input_dir);
    let writer = m::manifest_writer(output);
    serde_json::to_writer(writer, &manifest).expect("Failed to serialize manifest.");
}
