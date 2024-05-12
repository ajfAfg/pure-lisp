#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Id(String),
    LParen,
    RParen,
    Comma,
    Cond,
    Atom,
    Eq,
    Car,
    Cdr,
    Cons,
    Lambda,
    Apply,
    Label,
    Quote,
}
