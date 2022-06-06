use std::env;
extern crate cbindgen;
use cbindgen::Language::C;

fn main() {
    setup_cbindgen();
}

fn setup_cbindgen() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR");
    match crate_dir {
        Ok(val) => {
            cbindgen::Builder::new()
                .with_crate(val)
                .with_language(C)
                .generate()
                .expect("Unable to generate bindings")
                .write_to_file("include/glue_native.h");
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
