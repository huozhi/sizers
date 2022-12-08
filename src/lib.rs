use anyhow::Error;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::prelude::*;
use std::sync::Arc;
use swc_core::{
    base::{
        config::JsMinifyFormatOptions, config::JsMinifyOptions, try_with_handler, BoolOrDataConfig,
        Compiler,
    },
    ecma::minifier::option::terser::TerserEcmaVersion,
    common::{FileName, FilePathMapping, SourceMap, GLOBALS},
};

pub struct CompressOutput {
    pub origin: u128,
    pub gzipped: u128,
    pub minified: u128,
}

fn compiler() -> Arc<Compiler> {
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    Arc::new(Compiler::new(cm))
}

pub fn swc_minify(s: &str) -> Result<String, Error> {
    let c = compiler();

    try_with_handler(c.cm.clone(), Default::default(), |handler| {
        GLOBALS.set(&Default::default(), || {
            let opts = JsMinifyOptions {
                ecma: TerserEcmaVersion::Num(2018),
                module: false,
                safari10: false,
                keep_fnames: false,
                keep_classnames: false,
                inline_sources_content: false,
                toplevel: Default::default(),
                // ecma: Default::default(),
                mangle: Default::default(),
                output_path: Option::None,
                compress: BoolOrDataConfig::from_bool(true),
                source_map: BoolOrDataConfig::from_bool(false),
                format: JsMinifyFormatOptions::default(),
                emit_source_map_columns: Default::default(),
            };

            let fm = c.cm.new_source_file(FileName::Anon, s.into());
            let program = c.minify(fm, &handler, &opts).expect("failed to minify");

            Ok(program.code)
        })
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
