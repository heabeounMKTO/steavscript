use anyhow::{Error, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub enum BelteiMode {
    FromFile,
    Interpreter,
}

pub struct RunBeltei {
    pub mode: BelteiMode,
    pub buffer: String,
}

impl RunBeltei {
    pub fn run_file(&self) -> Result<(), Error> {
        let f = File::open(&self.buffer)?;
        let mut reader = BufReader::new(f);
        for line in reader.lines() {
            run(&line?).unwrap();
        }
        Ok(())
    }
    pub fn run_line(&self) -> Result<(), Error> {
        run(&self.buffer).unwrap();
        Ok(())
    }
}

fn run(ayylmao: &str) -> Result<(), Error> {
    println!("{:?}", ayylmao);
    Ok(())
}
