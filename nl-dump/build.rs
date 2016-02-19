use std::env;
use std::path::PathBuf;

extern crate rust_enum_derive;

fn main() {
    let output_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let input_dir = PathBuf::from("build_input/");

    let r = rust_enum_derive::traverse_dir(&input_dir, &output_dir);
    match r
    {
        Err(e) => {
            panic!("Error: {}", e);
        }
        Ok(_) => { }
    }
}
