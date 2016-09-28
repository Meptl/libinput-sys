extern crate bindgen;

use std::fs::File;
use std::io::Write;

const OUT_FILE: &'static str = concat!(env!("OUT_DIR"), "/libinput-sys.rs");

pub fn main() {
    let mut file = Box::new(File::create(OUT_FILE).expect("Unable to write outfile"));
    write!(&mut file, "pub mod ffi {{").unwrap();

    // hard coded library path
    let mut builder =  bindgen::Builder::new("/usr/include/libinput.h");
    builder.link("input", bindgen::LinkType::Dynamic);
    builder.link("udev", bindgen::LinkType::Dynamic);
    builder.builtins();

    let bindings = match builder.generate() {
        Ok(b) => b.to_string(),
        Err(e) => panic!(e)
    };

    file.write_all(bindings.into_bytes().as_slice()).expect("failed to write bindings");
    write!(&mut file, "}}").unwrap();
}
