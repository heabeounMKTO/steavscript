use crate::file_utils::loading::load_beltei_file;

use std::rc::Rc;
use regex::Regex;
use super::beltei_model::Token;

pub fn tokenize_beltei_file(input_file: &str) -> Vec<String> {
    // let loaded_file = loading::load_beltei_file(&input_file);
    // println!({":?"}, loaded_file);
    vec![String::from("ayylmao")]
}

pub fn tokenize_string(value: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut end_idx = 0;
    let byte_array = value.as_bytes();
    while end_idx < value,len(){
        let literal
    }


    tokens

}
