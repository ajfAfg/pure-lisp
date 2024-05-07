use pure_lisp::syntax::SExpression;
use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[derive(Clone, Debug, PartialEq, Eq)]
struct SExpressionWrapper(SExpression);

impl Arbitrary for SExpressionWrapper {
    fn arbitrary(g: &mut Gen) -> Self {
        if g.choose(&[true, false]).unwrap().clone() {
            SExpressionWrapper(SExpression::Symbol(String::arbitrary(g)))
        } else {
            SExpressionWrapper(SExpression::Pair(
                Box::new(SExpressionWrapper::arbitrary(g).0),
                Box::new(SExpressionWrapper::arbitrary(g).0),
            ))
        }
    }
}

/*********************************************
Testing
**********************************************/
#[cfg(test)]
mod property_based_tests {
    use crate::SExpressionWrapper;

    quickcheck! {
        fn halting(exp: SExpressionWrapper) -> bool {
            exp.clone().0.eval();
            true
        }

        fn uniqueness(exp: SExpressionWrapper) -> bool {
            exp.clone().0.eval() == exp.0.eval()
        }
    }
}

mod sample_based_tests {
    use pure_lisp::{
        environment::Environment,
        syntax::{SExpression, Term, Value},
    };

    fn env() -> Environment {
        Environment::new()
    }

    fn t() -> Box<Term> {
        Box::new(Term::Quote(SExpression::Symbol("T".to_string())))
    }
    fn f() -> Box<Term> {
        Box::new(Term::Quote(SExpression::Symbol("F".to_string())))
    }
    fn dummy_atom(id: &str) -> Box<Term> {
        Box::new(Term::Quote(SExpression::Symbol(
            format!("{}", id).to_string(),
        )))
    }
    fn dummy_pair(id1: &str, id2: &str) -> Box<Term> {
        Box::new(Term::Quote(SExpression::Pair(
            Box::new(SExpression::Symbol(format!("{}", id1))),
            Box::new(SExpression::Symbol(format!("{}", id2))),
        )))
    }
    fn target() -> Box<Term> {
        Box::new(Term::Quote(SExpression::Symbol("TARGET".to_string())))
    }

    fn t_value() -> Value {
        t().eval(&mut env()).unwrap()
    }
    fn f_value() -> Value {
        f().eval(&mut env()).unwrap()
    }
    fn target_value() -> Value {
        target().eval(&mut env()).unwrap()
    }

    #[test]
    fn cond() {
        // NOTE: c.f. p.4 in the paper
        assert_eq!(
            Term::Cond(vec![(t(), target()), (f(), dummy_atom(""))]).eval(&mut env()),
            Some(target_value())
        );

        assert_eq!(
            Term::Cond(vec![
                (f(), dummy_atom("1")),
                (t(), target()),
                (t(), dummy_atom("2"))
            ])
            .eval(&mut env()),
            Some(target_value())
        );

        assert_eq!(
            Term::Cond(vec![(f(), dummy_atom("")), (t(), target())]).eval(&mut env()),
            Some(target_value())
        );

        assert_eq!(
            Term::Cond(vec![
                (f(), Box::new(Term::Car(dummy_atom("")))),
                (t(), target())
            ])
            .eval(&mut env()),
            Some(target_value())
        );

        assert_eq!(
            Term::Cond(vec![
                (f(), dummy_atom("1")),
                (t(), Box::new(Term::Car(dummy_atom("2"))))
            ])
            .eval(&mut env()),
            None
        );

        assert_eq!(
            Term::Cond(vec![(f(), dummy_atom("1")), (f(), dummy_atom("2"))]).eval(&mut env()),
            None
        );
    }

    #[test]
    fn atom() {
        assert_eq!(Term::Atom(dummy_atom("")).eval(&mut env()), Some(t_value()));

        assert_eq!(
            Term::Atom(dummy_pair("1", "2")).eval(&mut env()),
            Some(f_value())
        );
    }

    #[test]
    fn eq() {
        assert_eq!(
            Term::Eq(dummy_atom(""), dummy_atom("")).eval(&mut env()),
            Some(t_value())
        );

        assert_eq!(
            Term::Eq(dummy_atom("1"), dummy_atom("2")).eval(&mut env()),
            Some(f_value())
        );

        assert_eq!(
            Term::Eq(dummy_atom(""), dummy_pair("1", "2")).eval(&mut env()),
            None
        );

        assert_eq!(
            Term::Eq(dummy_pair("1", "2"), dummy_pair("3", "4")).eval(&mut env()),
            None
        );
    }

    #[test]
    fn car() {
        assert_eq!(
            Term::Car(dummy_pair("1", "2")).eval(&mut env()),
            Some(Value::Symbol("1".to_string()))
        );

        assert_eq!(Term::Car(dummy_atom("")).eval(&mut env()), None);
    }

    #[test]
    fn cdr() {
        assert_eq!(
            Term::Cdr(dummy_pair("1", "2")).eval(&mut env()),
            Some(Value::Symbol("2".to_string()))
        );

        assert_eq!(Term::Car(dummy_atom("")).eval(&mut env()), None);
    }

    #[test]
    fn cons() {
        assert_eq!(
            Term::Cons(dummy_atom("1"), dummy_atom("2")).eval(&mut env()),
            Some(Value::Pair(
                Box::new(Value::Symbol("1".to_string())),
                Box::new(Value::Symbol("2".to_string()))
            ))
        );

        assert_eq!(
            Term::Cons(dummy_atom("1"), dummy_pair("2", "3")).eval(&mut env()),
            Some(Value::Pair(
                Box::new(Value::Symbol("1".to_string())),
                Box::new(Value::Pair(
                    Box::new(Value::Symbol("2".to_string())),
                    Box::new(Value::Symbol("3".to_string()))
                ))
            ))
        );
    }

    #[test]
    fn variable() {
        assert_eq!(
            {
                let mut env = Environment::new();
                env.extend("x".to_string(), target_value());
                Term::Variable("x".to_string()).eval(&mut env)
            },
            Some(target_value())
        );

        assert_eq!(Term::Variable("x".to_string()).eval(&mut env()), None);

        assert_eq!(
            {
                let mut env = Environment::new();
                env.extend("x".to_string(), t_value());
                env.extend("y".to_string(), target_value());
                env.extend("z".to_string(), t_value());
                Term::Variable("y".to_string()).eval(&mut env)
            },
            Some(target_value())
        );
    }

    #[test]
    fn lambda_apply() {
        assert_eq!(
            Term::Apply(
                Box::new(Term::Lambda(
                    vec!["x".to_string()],
                    Box::new(Term::Variable("x".to_string()))
                )),
                vec![target()]
            )
            .eval(&mut env()),
            Some(target_value())
        );

        // NOTE: Same variable names are shadowed.
        assert_eq!(
            Term::Apply(
                Box::new(Term::Lambda(
                    vec!["x".to_string()],
                    Box::new(Term::Apply(
                        Box::new(Term::Lambda(
                            vec!["x".to_string()],
                            Box::new(Term::Variable("x".to_string()))
                        )),
                        vec![target()]
                    ))
                )),
                vec![dummy_atom("")]
            )
            .eval(&mut env()),
            Some(target_value())
        );

        // NOTE: A closure has an environment.
        assert_eq!(
            Term::Apply(
                Box::new(Term::Lambda(
                    vec!["h".to_string()],
                    Box::new(Term::Apply(
                        Box::new(Term::Variable("h".to_string())),
                        vec![f()]
                    ))
                )),
                vec![Box::new(Term::Apply(
                    Box::new(Term::Lambda(
                        vec!["x".to_string()],
                        Box::new(Term::Lambda(
                            vec!["y".to_string()],
                            Box::new(Term::Cons(
                                Box::new(Term::Variable("x".to_string())),
                                Box::new(Term::Variable("y".to_string()))
                            ))
                        ))
                    )),
                    vec![t()]
                ))]
            )
            .eval(&mut env()),
            Some(Value::Pair(Box::new(t_value()), Box::new(f_value())))
        );
    }

    #[test]
    fn label() {
        assert_eq!(
            Term::Apply(
                Box::new(Term::Label(
                    "last".to_string(),
                    Box::new(Term::Lambda(
                        vec!["x".to_string()],
                        Box::new(Term::Cond(vec![
                            (
                                Box::new(Term::Atom(Box::new(Term::Variable("x".to_string())))),
                                Box::new(Term::Variable("x".to_string()))
                            ),
                            (
                                t(),
                                Box::new(Term::Apply(
                                    Box::new(Term::Variable("last".to_string())),
                                    vec![Box::new(Term::Cdr(Box::new(Term::Variable(
                                        "x".to_string()
                                    ))))]
                                ))
                            )
                        ]))
                    ))
                )),
                vec![Box::new(Term::Quote(SExpression::Pair(
                    Box::new(SExpression::Symbol("1".to_string())),
                    Box::new(SExpression::Pair(
                        Box::new(SExpression::Symbol("2".to_string())),
                        Box::new(SExpression::Symbol("3".to_string()))
                    ))
                )))]
            )
            .eval(&mut Environment::new()),
            Some(Value::Symbol("3".to_string()))
        );
    }
}
