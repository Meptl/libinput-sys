extern crate bindgen;

use std::env;
use std::path::PathBuf;

pub fn main() {
    // Tell cargo to tell rustc to link the system bzip2 shared library.
    println!("cargo:rustc-link-lib=input");
    println!("cargo:rustc-link-lib=udev");

    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .header("wrapper.h")
        .emit_builtins()
        .generate()
        .expect("Unable to generate bindings.");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("libinput-sys.rs"))
            .expect("Couldn't write bindings!");
}
