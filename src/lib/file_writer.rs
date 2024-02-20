use log::error;
use std::fs::File;
use std::io::prelude::*;

pub fn write_file(path: String, lines: Vec<String>) {
    let file_result = File::create(path.clone());
    if file_result.is_err() {
        error!("Could not write file: {}", path);
    }
    let ser: String = lines.join("\n");
    let write_result = file_result.unwrap().write_all(&ser.into_bytes());
    if write_result.is_err() {
        error!("Could not write file: {}", path);
    }
}
