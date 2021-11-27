use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;


pub struct CompressResult {
  pub raw: u128,
  pub gzip: u128
}

pub fn minify(code: &String) -> CompressResult {
  let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

  let raw_bytes = code.as_bytes();
  match encoder.write_all(raw_bytes) {
      Ok(_) => {},
      Err(e) => panic!("{:?}", e)
  }
  
  let compressed_bytes = match encoder.finish() {
      Ok(bytes) => bytes.len(),
      Err(e) => panic!("{:?}", e)
  };

  CompressResult {
    raw: raw_bytes.len() as u128,
    gzip: compressed_bytes as u128
  }
}
