extern crate rustfk;

use std::env;
use std::fs::File;
use std::io::Read;

use rustfk::RustFk;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).unwrap();

    let mut commands: Vec<u8> = vec![];
    f.read_to_end(&mut commands).unwrap();

    let mut interpreter = RustFk::new(40, commands);
    interpreter.run().unwrap();
}
