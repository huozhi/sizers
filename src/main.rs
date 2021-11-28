use std::env;
use std::fs;
use js_sizers::{compress};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading file: {}...\n", filename);

    let code = fs::read_to_string(filename).expect("Could not read file");
    let output = compress(&code);

    println!("origin: {} bytes", output.origin);
    println!("minified: {} bytes", output.minified);
    println!("gzipped: {} bytes", output.gzip);
}
