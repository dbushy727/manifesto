extern crate serde_json;

use std::collections::HashMap;

use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let args = &mut env::args();

    let manifest_json: &str = &args.nth(1)
        .expect("Missing manifest.json path.");

    let target_dir: &str = &args.nth(2)
        .unwrap_or("output/".to_string());

    let mut reader: Box<Read> = if manifest_json == "-" {
        Box::new(io::stdin())
    } else {
        let file = File::open(manifest_json)
            .expect("Manifest file not found.");
        Box::new(file)
    };

    let mut manifest_buf = Vec::new();
    io::copy(&mut reader, &mut manifest_buf)
        .expect("Unable to load manifest.");

    let manifest: HashMap<String, String> = serde_json::from_slice(&manifest_buf)
        .expect("Invalid manifest.");

    fs::create_dir_all(target_dir).expect("Could not create target dir.");

    for (file_name, new_name) in manifest {
        let new_file = [target_dir, &new_name].join("");
        fs::copy(file_name, new_file).expect("Could not rename file.");
    }
}
