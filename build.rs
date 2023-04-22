extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    let blockdev = pkg_config::Config::new()
        .probe("blockdev")
        .expect("libblockdev not found");
    let includes: Vec<String> = blockdev
        .include_paths
        .iter()
        .map(|path| format!("{}", path.to_string_lossy()))
        .collect();
    let libs: Vec<String> = blockdev
        .libs
        .iter()
        .map(|lib| format!("{}", lib.as_str()))
        .collect();
    // for include in includes.iter() {
    //     println!("cargo:rustc-link-search={}", include);
    // }
    // for lib in libs.iter() {
    //     println!("cargo:rustc-link-lib={}", lib);
    // }
    // readd link search bits so rustc knows how to link and not just bindgen
    println!("cargo:rustc-link-lib=bd_lvm");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(includes.iter().map(|path| format!("-I{}", path)))
        .clang_arg("-lbd_lvm")
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");
    // panic!("debug");
}
