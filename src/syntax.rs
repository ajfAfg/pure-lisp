use crate::environment::Environment;
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Symbol(String),
    Pair(Box<Value>, Box<Value>),
    Closure(Vec<String>, Box<Term>, Rc<RefCell<Environment>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SExpression {
    Symbol(String),
    Pair(Box<SExpression>, Box<SExpression>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    /* Conditional Expressions */
    Cond(Vec<(Box<Term>, Box<Term>)>),
    /* The Elementary S-functions and Predicates */
    Atom(Box<Term>),
    Eq(Box<Term>, Box<Term>),
    Car(Box<Term>),
    Cdr(Box<Term>),
    Cons(Box<Term>, Box<Term>),
    /* Functions and Forms */
    Variable(String),
    Lambda(Vec<String>, Box<Term>),
    Apply(Box<Term>, Vec<Box<Term>>),
    /* Expressions for Recursive Functions */
    Label(String, Box<Term>),
    /* M-expression */
    Quote(SExpression),
}
