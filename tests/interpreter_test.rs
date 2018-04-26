extern crate rustfk;

use std::io::Cursor;

use rustfk::RustFk;

#[test]
fn hello_world() {
    let cmd = str2vec("--[>--->->->++>-<<<<<-------]>--.>---------.>--..+++.>----.>+++++++++.<<.+++.------.<-.>>+.");

    let input: Vec<u8> = Vec::new();
    let mut output: Vec<u8> = Vec::new();
    let mut input_cursor = Cursor::new(input);

    let mut interpreter = RustFk::new(200, cmd);
    interpreter.run(&mut input_cursor, &mut output).unwrap();

    let output_content = String::from_utf8(output).unwrap();
    assert_eq!("Hello world!", output_content);
}

#[test]
fn head() {
    let cmd = str2vec("+>>>>>>>>>>-[,+[-.----------[[-]>]<->]<]");

    let input = str2vec("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n");
    let mut output: Vec<u8> = Vec::new();
    let mut input_cursor = Cursor::new(input);

    let mut interpreter = RustFk::new(200, cmd);
    interpreter.run(&mut input_cursor, &mut output).unwrap();

    let output_content = String::from_utf8(output).unwrap();
    assert_eq!("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n", output_content);
}

fn str2vec(s: &'static str) -> Vec<u8> {
    String::from(s).into_bytes()
}