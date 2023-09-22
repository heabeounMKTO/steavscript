mod bscript;
mod bscript_parser;
use bscript::implmode::{BelteiMode, RunBeltei};
use bscript_parser::read_bs::read_bscript;
use clap::Parser;
use input_stream::InputStream;
use std::io;
use std::io::BufRead;
#[derive(Parser, Debug)]
struct CliArguments {
    #[arg(long, default_value = "interp", required = false)]
    file: String,
}

fn main() {
    let parse_args = CliArguments::parse();
    match parse_args.file.as_str() {
        
        "interp" => loop {
            print!(">>"); 
            let stdin = io::stdin();
            let line = stdin.lock().lines().next().unwrap();
            let runner: RunBeltei = RunBeltei {
                mode: BelteiMode::Interpreter,
                buffer: line.unwrap(),
            };
            runner.run_line().unwrap();
        },
        _ => {
            let runner = RunBeltei {
                mode: BelteiMode::FromFile,
                buffer: parse_args.file,
            };
            runner.run_file().unwrap();
        }
    };
}
