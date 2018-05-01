extern crate manifesto as m;
extern crate serde_json;

use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().collect();
    let default_output = "output/".to_string();

    let input = args.iter().nth(1).expect("Missing manifest.json");
    let output = args.iter().nth(2).unwrap_or(&default_output);

    let reader = m::manifest_reader(input);

    let manifest: HashMap<String, String> =
        serde_json::from_reader(reader).expect("Invalid manifest.");

    fs::create_dir_all(output).expect("Could not create output directory.");

    for (file_name, new_name) in manifest {
        let new_file = [output.to_string(), new_name].join("");
        fs::copy(file_name, new_file).expect("Could not rename file.");
    }
}
