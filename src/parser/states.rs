extern crate itertools;

use parser::*;
use ascii;
use self::itertools::Itertools;
use parser::style::*;

pub struct Ground;
pub struct EscapeSeq;
pub struct ControlSequence;
pub struct SystemCommandSequence;
pub struct TwoCharSequence{first:char}

impl State for Ground {
    fn next(&self, emu: &VtParser) -> Option<Box<State>> {
        let ch:Option<char> = emu.stream.borrow_mut().next();
        match ch {
                Some(x)=>
                    match x {
                        ascii::BEL => emu.emit(Code::Bell),
                        ascii::BS  => emu.emit(Code::Backspace),
                        ascii::CR  => emu.emit(Code::CarriageReturn),
                        ascii::ENQ => emu.emit(Code::ReturnTerminalStatus),
                        ascii::FF | ascii::LF | ascii::VT  => emu.emit(Code::NewLine),
                        ascii::SI => emu.emit(Code::MapCharsetToGL(0)),
                        ascii::SO => emu.emit(Code::MapCharsetToGL(1)),
                        ascii::HT => emu.emit(Code::HorizontalTab),
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

        Some(Box::new(Ground))
    }
}

impl State for EscapeSeq {
    fn next(&self, emu: &VtParser) -> Option<Box<State>> {
        let ch:Option<char> = emu.stream.borrow_mut().next();
        match ch {
                Some(x)=>
                    match x {
                        '[' => return Some(Box::new(ControlSequence)),
                        'D' => emu.emit(Code::Index),
                        'E' => emu.emit(Code::NextLine),
                        'H' => emu.emit(Code::HorizontalTabSet),
                        'M' => emu.emit(Code::ReverseIndex),
                        'N' => emu.emit(Code::SingleShiftSelect(2)),
                        'O' => emu.emit(Code::SingleShiftSelect(3)),
                        ']' => return Some(Box::new(SystemCommandSequence)),
                        '6' => emu.emit(Code::BackIndex),
                        '7' => emu.emit(Code::SaveCursor),
                        '8' => emu.emit(Code::RestoreCursor),
                        '9' => emu.emit(Code::ForwardIndex),
                        '=' => emu.emit(Code::ApplicationKeypad),
                        '>' => emu.emit(Code::NormalKeypad),
                        'F' => emu.emit(Code::CursorToLowerLeft),
                        'c' => emu.emit(Code::FullReset),
                        'n' => emu.emit(Code::MapCharsetToGL(2)),
                        'o' => emu.emit(Code::MapCharsetToGL(3)),
                        '|' => emu.emit(Code::MapCharsetToGR(3)),
                        '}' => emu.emit(Code::MapCharsetToGR(2)),
                        '~' => emu.emit(Code::MapCharsetToGL(1)),
                        '#' | '(' | ')' |'*' | '+' | '$' | '@' | '%' | '.' | '/' | ' ' =>
                                return Some(Box::new(TwoCharSequence{first:x})),
                        _ => {}
                    },
                None => return None
        }


        Some(Box::new(Ground))
    }
}


impl State for ControlSequence {
    fn next(&self, emu: &VtParser) -> Option<Box<State>> {
        let mut pos = -1;
        let mut question_mark = false;
        let mut more_mark = false;
        let mut final_char: char = 0 as char;

        let mut argv: Vec<u32> = Vec::new();
        let mut cur: u32 = 0;
        let mut digit = false;

        loop {
            let ch:Option<char> = emu.stream.borrow_mut().next();
            match ch {
                Some(x) => {
                    pos+=1;
                    match x {
                        '?' => if pos == 0 {
                                    question_mark = true;
                               },
                        '>' => if pos == 0 {
                                   more_mark = true;
                               },
                        ';' => if digit {
                                    argv.push(cur);
                                    cur = 0;
                                    digit = false;
                                },
                        '0'...'9' => {
                            cur = 10*cur + (x as u32) - ('0' as u32);
                            digit = true;
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

        let arg1 = |i: usize, default: u32| -> u32 {
            if i<argv.len() && argv[i] != 0 {argv[i]} else {default}
        };

        match final_char {
            '@' => emu.emit(Code::InsertBlankCharacters(arg(0, 1))),
            'A' => emu.emit(Code::CursorUp(arg1(0, 1))),
            'B' => emu.emit(Code::CursorDown(arg1(0, 1))),
            'C' => emu.emit(Code::CursorForward(arg1(0, 1))),
            'D' => emu.emit(Code::CursorBackward(arg1(0, 1))),
            'E' => emu.emit(Code::CursorNextLine(arg1(0, 1))),
            'F' => emu.emit(Code::CursorPrecedingLine(arg1(0, 1))),
            'G' | '`' => emu.emit(Code::CursorHorizontalAbsolute(arg(0, 1))),
            'f' | 'H' => emu.emit(Code::CursorPosition{x:arg(0, 1), y:arg(1, 1)}),
            'J' => emu.emit(if question_mark {Code::SelectiveEraseInDisplay(arg(0, 1))} else {Code::EraseInDisplay(arg(0, 1))}),
            'K' => emu.emit(if question_mark {Code::SelectiveEraseInLine(arg(0, 1))} else {Code::EraseInLine(arg(0, 1))}),
            'L' => emu.emit(Code::InsertLines(arg(0, 1))),
            'M' => emu.emit(Code::DeleteLines(arg(0, 1))),
            'X' => emu.emit(Code::EraseCharacters(arg(0, 1))),
            'P' => emu.emit(Code::DeleteCharacters(arg(0, 1))),
            'S' => emu.emit(Code::ScrollUp(arg(0, 1))),
            'T' => emu.emit(Code::ScrollDown(arg(0, 1))),
            'c' => emu.emit(if more_mark && arg(0, 1) == 0 {Code::SendSecondaryDeviceAttributes} else {Code::SendPrimaryDeviceAttributes}),
            'd' => emu.emit(Code::LinePositionAbsolute(arg(0, 1))),
            'g' => emu.emit(Code::TabClear(arg(0, 1))),
            'h' => emu.emit(Code::SetMode(arg(0, 1))), //TODO
            'l' => emu.emit(Code::SetMode(arg(0, 1))), //TODO
            'm' => emit_style_codes(emu, &argv),
            'n' => emu.emit(if question_mark {Code::DecDeviceStatusReport} else {Code::DeviceStatusReport(arg(0, 1))}),
            'r' => emu.emit(if question_mark {Code::RestoreDecPrivateMode} else {Code::SetScrollingRegion{top:arg(0, 1), bottom:arg(1, 0)}}),
            't' => match arg(0, 0) {
                        8 => emu.emit(Code::Resize{width: arg(1, 0), height:arg(2, 0)}),
                        e@_ => emu.error_msg(format!("Unkown window manipulation command {}", e.to_string()))
                    },
            _ => emu.error_msg(format!("Unknown CSI code {}", final_char))
        };

        Some(Box::new(Ground))
    }
}

impl State for SystemCommandSequence {
    fn next(&self, emu: &VtParser) -> Option<Box<State>> {
        let mut argv: Vec<String> = Vec::new();
        let mut s = String::new();
        loop {
            let ch:Option<char> = emu.stream.borrow_mut().next();
            match ch {
                Some(x) => {
                    match x {
                        ';' | ascii::ST | ascii::BEL => {
                            argv.push(s);
                            if x == ';' {
                                s = String::new();
                            } else {
                                break;
                            }
                        },
                        _ => s.push(x)
                    }
                },
                _ => {
                    argv.push(s);
                    break;
                }
            }
        }

        if argv.len() > 1 {
            match argv[0].parse::<u32>() {
                Ok(x) =>
                match x {
                    0 | 2 => emu.emit(Code::WindowTitle(argv[1].to_string())),
                    7 => emu.emit(Code::CurrentPath(argv[1].to_string())),
                    _ => emu.error_msg("Unrecognized system command ".to_string())
                },
                Err(_) => { emu.error_msg("Cant parse system command".to_string()) }
            }
        }

        Some(Box::new(Ground))
    }

}

impl State for TwoCharSequence {
    fn next(&self, emu: &VtParser) -> Option<Box<State>> {
        let ch:Option<char> = emu.stream.borrow_mut().next();
        let a = self.first;
        match ch {
                Some(b)=>
                    match a {
                        ' ' => match b {
                            'F' => emu.emit(Code::Charset7Bit),
                            'G' => emu.emit(Code::Charset8Bit),
                            'L' => emu.emit(Code::AnsiConformanceLevel(1)),
                            'M' => emu.emit(Code::AnsiConformanceLevel(2)),
                            'N' => emu.emit(Code::AnsiConformanceLevel(3)),
                            _   => emu.error(&[a, b])
                        },
                        '#' => match b {
                            '8' => emu.emit(Code::FillScreenE),
                            _   => emu.error(&[a, b])
                        },
                        '%' => match b {
                            '@' => emu.emit(Code::SelectDefaultCharset),
                            'G' => emu.emit(Code::SelectUtf8Charset),
                            _   => emu.error(&[a, b])
                        },
                        '(' => emu.emit(Code::DesignateCharset(0, b)), //Designate G0 Character set (VT100)
                        ')' => emu.emit(Code::DesignateCharset(1, b)), //Designate G1 Character set (VT100)
                        '*' => emu.emit(Code::DesignateCharset(2, b)), //Designate G2 Character set (VT220)
                        '+' => emu.emit(Code::DesignateCharset(3, b)), //Designate G3 Character set (VT220)
                        '-' => emu.emit(Code::DesignateCharset(1, b)), //Designate G1 Character set (VT300)
                        '.' => emu.emit(Code::DesignateCharset(2, b)), //Designate G2 Character set (VT300)
                        '/' => emu.emit(Code::DesignateCharset(3, b)), //Designate G3 Character set (VT300)

                        _ => emu.error(&[a, b])
                    },
                None => emu.error(&[a])

        };

        Some(Box::new(Ground))
    }
}
