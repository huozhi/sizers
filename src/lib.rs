use flate2::Compression;
use std::io::prelude::*;
use anyhow::{Error};
use flate2::write::ZlibEncoder;
use std::sync::Arc;
use swc::config::util::{BoolOrObject};
use swc::common::{FileName, FilePathMapping, SourceMap};
use swc::{
  config::{JsMinifyOptions, JsMinifyFormatOptions},
  try_with_handler, Compiler,
};
use swc::ecmascript::minifier::option::{
  MangleOptions,
  terser::{TerserEcmaVersion}
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
      module: false,
      safari10: false,
      toplevel: false,
      keep_fnames: false,
      keep_classnames: false,
      inline_sources_content: false,
      output_path: Option::None,
      compress: BoolOrObject::Bool(true),
      ecma: TerserEcmaVersion::Num(2016),
      source_map: BoolOrObject::Bool(false),
      format: JsMinifyFormatOptions::default(),
      mangle: BoolOrObject::Obj(MangleOptions { top_level: true, ..Default::default() }),
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
  
  encoder.write_all(minified_bytes).unwrap();

  let gzipped_bytes = encoder.finish().map(|bytes| bytes.len()).unwrap();

  CompressOutput {
    origin: code.as_bytes().len() as u128,
    minified: minified_bytes.len() as u128,
    gzipped: gzipped_bytes as u128,
  }
}
