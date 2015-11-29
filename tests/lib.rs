extern crate vt100;

use vt100::parser::*;
use vt100::ascii;

use std::str::Chars;

fn do_test(stream: Chars)->Vec<Code> {
    VtParser::new(stream).collect()
}

fn do_test_esc(seq: &str)->Vec<Code> {
    let mut s = String::new();
    s.push(ascii::ESC);
    s.push_str(seq);
    do_test(s.chars())
}

#[test]
fn test_beep() {
    let mut s = "Hello world!".to_string();
    s.push(ascii::BEL);
    assert_eq!(do_test(s.chars()), [Code::Chars("Hello world!".to_string()), Code::Bell]);
}

#[test]
fn test_insert_blank_characters() {
    assert_eq!(do_test_esc("[3;@"), [Code::InsertBlankCharacters(3)]);
}
