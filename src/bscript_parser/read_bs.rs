use std::fs;

pub fn read_bscript(file_path: &str) -> String {
    fs::read_to_string(file_path).expect(&format!("error reading file {:?}", &file_path))
}
