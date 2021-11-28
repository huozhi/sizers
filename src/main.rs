use js_sizers::compress;
use pretty_bytes::converter::convert;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let flag = args.get(2).map(|s| s.as_str()).unwrap_or("");

    println!("Reading file: {}...\n", filename);

    let code = fs::read_to_string(filename).expect("Could not read file");
    let output = compress(&code);

    match flag {
        "--pretty" | "-p" => {
            println!("origin: {}", convert(output.origin as f64));
            println!("minified: {}", convert(output.minified as f64));
            println!("gzipped: {}", convert(output.gzip as f64));
        }
        _ => {
            println!("origin: {} bytes", output.origin);
            println!("minified: {} bytes", output.minified);
            println!("gzipped: {} bytes", output.gzip);
        }
    }
}
