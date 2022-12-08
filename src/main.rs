use js_sizers::{compress, CompressOutput};
use pretty_bytes::converter::convert;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Processing: {}...\n", filename);

    let code = fs::read_to_string(filename).expect("Could not read file");
    let output = compress(&code);
    let CompressOutput {
        origin,
        minified,
        gzipped,
    } = output;

    println!(
        "\x1b[36m{}\x1b[0m   >> {} ({} bytes)",
        "origin",
        convert(origin as f64),
        origin
    );
    println!(
        "\x1b[36m{}\x1b[0m >> {} ({} bytes)",
        "minified",
        convert(minified as f64),
        minified
    );
    println!(
        "\x1b[36m{}\x1b[0m  >> {} ({} bytes)",
        "gzipped",
        convert(gzipped as f64),
        gzipped
    );
}
