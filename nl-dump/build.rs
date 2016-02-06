use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let r = Command::new("rust-enum-derive").args(&["--input_dir", "build_input/", "--output_dir"])
                                            .arg(&format!("{}", out_dir)).status();
    match r
    {
        Err(e) => {
            let i = e.raw_os_error();
            match i {
                Some(r) => {
                    if r == 2 {
                        panic!("You need to add rust-enum-derive to your path!");
                    }
                }
                _ => ()
            }
            panic!("Error: {} (check to see if rust-enum-derive is in your path)", e);
        }
        Ok(status) => {
            if status.code().unwrap() != 0 {
                panic!("Error: {}", status);
            }
        }
    }
}
