use std::fs::File;
use std::io::prelude::*;
use anyhow::Result;
use std::path::Path;
use std::ffi::OsStr;


pub fn load_beltei_file(input_path: &str) -> Result<String> { 
    check_for_beltei_extension(&input_path);
    let mut bt_file = File::open(&input_path)?;
    let mut contents = String::new();
    bt_file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn check_for_beltei_extension(input_path: &str) -> () {
    let extension = Path::new(input_path).extension().and_then(OsStr::to_str).unwrap(); 
    let beltei_extension = String::from("beltei");
    let is_beltei = assert_eq!(&extension, &beltei_extension, 
               "checking if its a beltei file ayylmao",
                );
    is_beltei
}

