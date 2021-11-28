use flate2::Compression;
use std::io::prelude::*;
use anyhow::{Error};
use flate2::write::ZlibEncoder;
use std::sync::Arc;
use swc::config::util::{BoolOrObject};
use swc::common::{FileName, FilePathMapping, SourceMap};
use swc::ecmascript::minifier::option::{MangleOptions};
use swc::{
  config::{JsMinifyOptions},
  try_with_handler, Compiler,
};

pub struct CompressOutput {
  pub origin: u128,
  pub gzipped: u128,
  pub minified: u128
}

fn compiler() -> Arc<Compiler> {
  let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
  Arc::new(Compiler::new(cm))
}

pub fn swc_minify(s: &str) -> Result<String, Error> {
  let c = compiler();

  try_with_handler(c.cm.clone(), false, |handler| {
    let opts = JsMinifyOptions {
      mangle: BoolOrObject::Obj(MangleOptions {
        top_level: true,
        ..Default::default()
      }),
      ..Default::default()
    };
    

    let fm = c.cm.new_source_file(FileName::Anon, s.into());
    let program = c
      .minify(fm, &handler, &opts)
      .expect("failed to minify");
    
    Ok(program.code)
  })
}

pub fn compress(code: &String) -> CompressOutput {
  let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
  let minified_code = swc_minify(&code).expect("failed to minify");
  let minified_bytes = minified_code.as_bytes();
  
  match encoder.write_all(minified_bytes) {
    Ok(_) => {}
    Err(e) => panic!("{:?}", e),
  }

  let gzipped_bytes = match encoder.finish() {
    Ok(bytes) => bytes.len(),
    Err(e) => panic!("{:?}", e),
  };

  CompressOutput {
    origin: code.as_bytes().len() as u128,
    minified: minified_bytes.len() as u128,
    gzipped: gzipped_bytes as u128,
  }
}
