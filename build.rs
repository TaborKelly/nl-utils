use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("rust-enum-derive").args(&["--input_dir", "build_input/", "--output_dir"])
                                    .arg(&format!("{}", out_dir)).status().unwrap();
}
