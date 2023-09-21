mod bscript;
mod bscript_parser;
use clap::Parser;
use bscript_parser::read_bs::read_bscript;
use bscript::implmode::{RunBeltei, BelteiMode};
use std::io;
use input_stream::InputStream;
#[derive(Parser, Debug)]
struct CliArguments {
    #[arg(long, default_value="interp", required=false)]
    file: String,
}

fn main() {
    let parse_args = CliArguments::parse();
    match parse_args.file.as_str() {
        "interp" => {
            let stdin = io::stdin();
            let mut input = InputStream::new(stdin.lock());
            let runner: RunBeltei = RunBeltei{
                mode: BelteiMode::Interpreter,
                buffer: input.scan().expect("cannot read from stdio")
            }; 
            runner.run_line().unwrap();
        },
        _ => {
           let runner = RunBeltei{
                mode: BelteiMode::FromFile,
                buffer: parse_args.file            
            };
            runner.run_file().unwrap();
        }
    };
}
