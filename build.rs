extern crate cbindgen;

use cbindgen::*;
use std::env;

fn new_base_builder() -> Builder {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    return Builder::new().with_crate(crate_dir);
}
fn main() {
    new_base_builder()
        .with_language(Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("extern/c/libmus.h");

    new_base_builder()
        .with_language(Language::Cxx)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("extern/cpp/libmus.hpp");
}
