extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/bpf/sdmm_malloc.o");
    println!("cargo:rustc-link-arg=src/bpf/sdmm_malloc.o");

    let bindings = bindgen::Builder::default()
        .header("src/bpf/wrapper.h")  
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}