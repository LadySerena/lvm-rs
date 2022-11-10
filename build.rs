extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=/usr/include/blockdev");
    println!("cargo:rustc-link-search=/usr/include/glib-2.0");
    println!("cargo:rustc-link-search=/usr/lib64/glib-2.0/include");


    println!("cargo:rustc-link-lib=blockdev");
    println!("cargo:rustc-link-lib=bd_lvm");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/include/glib-2.0")
        .clang_arg("-I/usr/lib64/glib-2.0/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");

}