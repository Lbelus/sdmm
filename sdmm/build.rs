extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bpf_header = "src/bpf/wrapper.h";
    let bindings = bindgen::Builder::default()
        .header(bpf_header)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
