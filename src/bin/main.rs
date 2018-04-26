extern crate rustfk;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

use rustfk::RustFk;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("error parsing arguments: {}", err);
        process::exit(1);
    });

    let mut f = File::open(config.filename).unwrap();

    let mut commands: Vec<u8> = vec![];
    f.read_to_end(&mut commands).unwrap();

    let mut interpreter = RustFk::new(40, commands);
    interpreter.run().unwrap_or_else(|err| {
        eprintln!("error runnning program: {:?}", err);
        process::exit(1);
    });
}

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config {
            filename,
        } )
    }
}