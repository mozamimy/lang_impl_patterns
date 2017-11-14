extern crate ll1_lexer;
extern crate ll1_parser;

use ll1_lexer::list_lexer;
use ll1_parser::list_parser;

use std::env;

fn main() {
    drive(env::args().nth(1).unwrap());
    println!("Syntax is OK.")
}

fn drive(input: String) {
    let lexer = list_lexer::ListLexer::new(input);
    let mut parser = list_parser::ListParser::new(lexer);

    parser.list();
}

#[test]
fn test_drive_1() {
    drive(String::from("[usagi, neko]"));
}

#[test]
fn test_drive_2() {
    drive(String::from("[usagi, [neko, dog], inko]"));
}

#[test]
fn test_drive_3() {
    drive(String::from("[usagi]"));
}

#[test]
#[should_panic(expected = "expecting LBrack")]
fn test_drive_4() {
    drive(String::from("]a,b]"));
}

#[test]
#[should_panic(expected = "expecting RBrack")]
fn test_drive_5() {
    drive(String::from("[a b]"));
}

#[test]
#[should_panic(expected = "expecting RBrack")]
fn test_drive_6() {
    drive(String::from("[a, [b, c"));
}
