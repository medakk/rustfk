extern crate rustfk;

use std::env;
use std::process;

use rustfk::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("error parsing arguments: {}\n", err);
        process::exit(1);
    });

    config.run().unwrap_or_else(|err| {
        eprintln!("error runnning program: {:?}", err);
        process::exit(1);
    });
}