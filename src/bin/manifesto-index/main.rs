extern crate md5;
extern crate serde_json;
extern crate walkdir;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::Read;
use std::str;

use walkdir::{DirEntry, WalkDir};
use md5::Digest;

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

        let md = fs::metadata(path).unwrap();
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
    let dir: &str = &env::args().nth(1).expect("Missing input directory.");

    let manifest = dir_to_manifest(&dir).expect("Could not build manifest.");
    let manifest_json = serde_json::to_string(&manifest).expect("Failed to serialize manifest.");

    println!("{}", manifest_json);
}
