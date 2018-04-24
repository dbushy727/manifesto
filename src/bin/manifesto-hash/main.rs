extern crate serde_json;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let args = &mut env::args();

    let manifest_json: &str = &args.nth(1)
        .expect("Missing manifest.json path.");
    let target_dir: &str = &args.nth(2)
        .unwrap_or("output/".to_string());

    let manifest_path = Path::new(&manifest_json);

    let mut manifest_file = File::open(manifest_path)
        .expect("Manifest file not found.");
    let mut manifest_str = String::new();
    manifest_file.read_to_string(&mut manifest_str)
        .expect("Unable to read manifest file.");

    let manifest: HashMap<String, String> = serde_json::from_str(&manifest_str)
        .expect("Invalid manifest.");

    fs::create_dir_all(target_dir).expect("Could not create target dir.");

    for (file_name, new_name) in manifest {
        let new_file = [target_dir, &new_name].join("");
        fs::copy(file_name, new_file).expect("Could not rename file.");
    }
}
