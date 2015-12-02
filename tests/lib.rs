extern crate vt100;

use vt100::parser::*;
use vt100::ascii;
use vt100::parser::style::*;

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
fn test_bell() {
    let mut s = "Hello world!".to_string();
    s.push(ascii::BEL);
    assert_eq!(do_test(s.chars()), [Code::Chars("Hello world!".to_string()), Code::Bell]);
}

#[test]
fn test_insert_blank_characters() {
    assert_eq!(do_test_esc("[31;@"), [Code::InsertBlankCharacters(31)]);
}

#[test]
fn test_system_commands() {
    assert_eq!(do_test(format!("{}]2;Rust{}", ascii::ESC, ascii::ST).chars()), [Code::WindowTitle("Rust".to_string())]);
}

#[test]
fn test_style() {
    assert_eq!(do_test_esc("[4;22;38;2;0;0;0;47;m"),
        [
            Code::StyleOption(Style::Underlined, true),
            Code::StyleOption(Style::Bold, false),
            Code::StyleOption(Style::Dim, false),
            Code::Foreground(Color::Rgb { r: 0, g: 0, b: 0 }),
            Code::Background(Color::Indexed(7))
        ]
    );
}
