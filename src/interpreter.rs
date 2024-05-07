use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    environment::Environment,
    syntax::{SExpression, Term, Value},
};

impl SExpression {
    pub fn eval(self) -> Value {
        match self {
            SExpression::Symbol(name) => Value::Symbol(name),
            SExpression::Pair(sexp1, sexp2) => {
                Value::Pair(Box::new((*sexp1).eval()), Box::new((*sexp2).eval()))
            }
        }
    }
}

impl Term {
    pub fn eval(self, env: &mut Environment) -> Option<Value> {
        let t = Value::Symbol("T".to_string());
        let f = Value::Symbol("F".to_string());

        match self {
            /* Conditional Expressions */
            Term::Cond(pairs) => {
                for (term1, term2) in pairs {
                    if (*term1).eval(env)? == t {
                        return Some((*term2).eval(env)?);
                    }
                }

                None
            }
            /* The Elementary S-functions and Predicates */
            Term::Atom(term) => match (*term).eval(env)? {
                Value::Symbol(_) => Some(t),
                _ => Some(f),
            },
            Term::Eq(term1, term2) => match ((*term1).eval(env)?, (*term2).eval(env)?) {
                (Value::Symbol(str1), Value::Symbol(str2)) => {
                    if str1 == str2 {
                        Some(t)
                    } else {
                        Some(f)
                    }
                }
                _ => None,
            },
            Term::Car(term) => match (*term).eval(env)? {
                Value::Pair(fst, _) => Some(*fst),
                _ => None,
            },
            Term::Cdr(term) => match (*term).eval(env)? {
                Value::Pair(_, snd) => Some(*snd),
                _ => None,
            },
            Term::Cons(term1, term2) => Some(Value::Pair(
                Box::new((*term1).eval(env)?),
                Box::new((*term2).eval(env)?),
            )),
            /* Functions and Forms */
            Term::Variable(name) => env.lookup(&name).map(|var| var.clone()),
            Term::Lambda(params, body) => Some(Value::Closure(
                params,
                body,
                Rc::new(RefCell::new(env.clone())),
            )),
            Term::Apply(term, terms) => match (*term).eval(env)? {
                Value::Closure(params, body, env2) => {
                    if params.len() == terms.len() {
                        let env2 = Rc::new(RefCell::new(env2.borrow().clone()));
                        let mut new_env = env2.borrow_mut();
                        for (param, term) in params.iter().zip(terms.iter()) {
                            new_env.extend(param.clone(), (*term.clone()).eval(env)?);
                        }
                        (*body).eval(&mut new_env)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            /* Expressions for Recursive Functions */
            Term::Label(name, term) => match *term {
                Term::Lambda(params, body) => {
                    let new_env = Rc::new(RefCell::new(env.clone()));
                    let value = Value::Closure(params, body, Rc::clone(&new_env));
                    new_env.borrow_mut().extend(name, value.clone());
                    Some(value)
                }
                _ => None,
            },
            /* Functions and Forms */
            Term::Quote(sexp) => Some(sexp.eval()),
        }
    }
}
