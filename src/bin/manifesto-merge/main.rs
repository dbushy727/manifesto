extern crate manifesto as m;
extern crate serde_json;

use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let default_output = "manifest.json".to_string();

    let manifest_a = args.iter().nth(1).expect("Missing first manifest.json");
    let manifest_b = args.iter().nth(2).expect("Missing second manifest.json");
    let output = args.iter().nth(3).unwrap_or(&default_output);

    let manifest_a = m::manifest_reader(manifest_a);
    let manifest_b = m::manifest_reader(manifest_b);

    let mut manifest_a: HashMap<String, String> =
        serde_json::from_reader(manifest_a).expect("Could not load first manifest.");
    let manifest_b: HashMap<String, String> =
        serde_json::from_reader(manifest_b).expect("Could not load second manifest.");

    manifest_a.extend(manifest_b);

    let writer = m::manifest_writer(output);
    serde_json::to_writer(writer, &manifest_a).expect("Failed to serialize manifest.");
}
