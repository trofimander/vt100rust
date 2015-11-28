#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Beep,
    Backspace,
    Chars(String)
}
