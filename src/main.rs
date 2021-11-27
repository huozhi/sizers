use std::env;
use std::fs;
use sizers::{minify, CompressResult};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading file: {}...", filename);

    let code = fs::read_to_string(filename).expect("Could not read file");
    let compress_result = minify(&code);

    let CompressResult { raw: raw_bytes, gzip: compressed_bytes } = compress_result;
    println!("raw bytes {}", raw_bytes);
    println!("gzip bytes {}", compressed_bytes);
}
