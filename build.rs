extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    let conf = pkg_config::Config::new();
    let blockdev = conf.probe("blockdev").unwrap();
    println!("cargo:rustc-link-lib=bd_lvm");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(
            blockdev
                .include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy())),
        )
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");
}
