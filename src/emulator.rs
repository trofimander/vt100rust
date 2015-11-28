extern crate itertools;

use self::itertools::Itertools;

use std::collections::VecDeque;
use std::str::Chars;
use std::cell::RefCell;
use std::rc::Rc;

use terminal::Term;
use ascii;

pub struct VtEmulator<'a> {
    term_actions: Rc<RefCell<VecDeque<Term>>>,
    stream: Rc<RefCell<Chars<'a>>>,
    state: Box<State>
}

impl<'a> VtEmulator<'a> {
    pub fn new(stream: Chars<'a>) -> VtEmulator<'a> {
        VtEmulator {
            term_actions: Rc::new(RefCell::new(VecDeque::new())),
            stream: Rc::new(RefCell::new(stream)),
            state: Box::new(State0)
        }
    }

    pub fn emit(&self, term:Term) {
        self.term_actions.borrow_mut().push_back(term);
    }

    pub fn get(&self)->Option<Term> {
        self.term_actions.borrow_mut().pop_front().clone()
    }

}

impl<'a> Iterator for VtEmulator<'a> {
    type Item = Term;

    fn next(&mut self) -> Option<Term> {
        loop {
            match self.get() {
                Some(x) => {
                    return Some(x)
                },
                None => match self.state.next(self) {
                    Some(x) => self.state = x,
                    None => break
                }
            }

        }

        None
    }
}

trait State {
    fn next(&self, emu: &VtEmulator) -> Option<Box<State>>;
}

struct State0;

impl State for State0 {
    fn next(&self, emu: &VtEmulator) -> Option<Box<State>> {
        let ch:Option<char> = emu.stream.borrow_mut().next();
        match ch {
                Some(x)=>
                    match x {
                        ascii::BEL => emu.emit(Term::Beep),
                        ascii::BS  => emu.emit(Term::Backspace),
                        _ => {
                            let chars: String =
                            emu.stream.borrow_mut().
                                take_while_ref(|x:&char| *x > 20u8 as char).collect();
                            let mut s = x.to_string();
                            s.push_str(&chars);
                            emu.emit(Term::Chars(s));
                        }
                    },
                    None => return None
        }

        Some(Box::new(State0))
    }
}
