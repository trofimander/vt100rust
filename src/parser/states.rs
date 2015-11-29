extern crate itertools;

use parser::*;
use ascii;
use self::itertools::Itertools;

pub struct State0;
pub struct EscapeSeq;
pub struct ControlSequence;

impl State for State0 {
    fn next(&self, emu: &VtParser) -> Option<Box<State>> {
        let ch:Option<char> = emu.stream.borrow_mut().next();
        match ch {
                Some(x)=>
                    match x {
                        ascii::BEL => emu.emit(Code::Bell),
                        ascii::BS  => emu.emit(Code::Backspace),
                        ascii::ESC => return Some(Box::new(EscapeSeq)),
                        _ => {
                            let chars: String =
                            emu.stream.borrow_mut().
                                take_while_ref(|x:&char| *x > 20u8 as char).collect();
                            let mut s = x.to_string();
                            s.push_str(&chars);
                            emu.emit(Code::Chars(s));
                        }
                    },
                    None => return None
        }

        Some(Box::new(State0))
    }
}

impl State for EscapeSeq {
    fn next(&self, emu: &VtParser) -> Option<Box<State>> {
        let ch:Option<char> = emu.stream.borrow_mut().next();
        match ch {
                Some(x)=>
                    match x {
                        '[' => return Some(Box::new(ControlSequence)),
                        _ => {}
                    },
                None => return None
        }


        Some(Box::new(State0))
    }
}


impl State for ControlSequence {
    fn next(&self, emu: &VtParser) -> Option<Box<State>> {
        let mut pos = -1;
        let mut starts_with_question_mark = false;
        let mut starts_with_more_mark = false;
        let mut final_char: char = 0 as char;

        let mut argv: Vec<u32> = Vec::new();
        let mut cur: u32 = 0;
        let mut digit: u8 = 0;

        loop {
            let ch:Option<char> = emu.stream.borrow_mut().next();
            match ch {
                Some(x) => {
                    pos+=1;
                    match x {
                        '?' => if pos == 0 {
                                    starts_with_question_mark = true;
                               },
                        '>' => if pos == 0 {
                                   starts_with_more_mark = true;
                               },
                        ';' => if (cur>0) {
                                    argv.push(cur);
                                    cur = 0;
                                },
                        '0'...'9' => {
                            cur = 10*cur + (x as u32) - ('0' as u32);
                        },
                        '\x40'...'\x7E' => {
                            final_char = x;
                        }
                        _ => {} //TODO: handle unhandled
                    }
                },
                None => {break}//TODO: error handling
            }
        }

        let arg = |i: usize, default: u32| -> u32 {
            if i<argv.len() {argv[i]} else {default}
        };

        match final_char {
            '@' => emu.emit(Code::InsertBlankCharacters(arg(0, 1))),
            _ => {} //TODO: handle
        };

        Some(Box::new(State0))
    }
}
