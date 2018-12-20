// build.rs

use std::process::Command;
use std::path::Path;

fn main() {
    if !Path::new("..\\gcc_wrapper\\target\\debug\\gcc_wrapper.exe").exists() {
        Command::new("cargo")
            .args(&["build"])
            .current_dir("..\\gcc_wrapper")
            .status()
            .unwrap();
    }
 }