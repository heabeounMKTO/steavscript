use std::env;
mod file_utils;
mod lexing;
use file_utils::loading;




fn main() {
    let test = loading::load_beltei_file("beltei_test/main.beltei");
    println!("{:?}", test.unwrap());
}
