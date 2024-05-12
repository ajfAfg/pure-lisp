mod sample_based_tests {
    use pure_lisp::{
        lexer::Lexer,
        parser::Parser,
        syntax::{SExpression, Term},
    };

    fn create_parser(s: &str) -> Parser {
        Parser::new(Lexer::new(&s.to_string()))
    }

    #[test]
    fn cond() {
        assert_eq!(
            create_parser("(cond ((atom x) x) (t y))").parse().unwrap(),
            Term::Cond(vec![
                (
                    Box::new(Term::Atom(Box::new(Term::Variable("x".to_string())))),
                    Box::new(Term::Variable("x".to_string()))
                ),
                (
                    Box::new(Term::Variable("t".to_string())),
                    Box::new(Term::Variable("y".to_string()))
                )
            ])
        );

        assert_eq!(create_parser("(cond)").parse().unwrap(), Term::Cond(vec![]));
    }

    #[test]
    fn atom() {
        assert_eq!(
            create_parser("(atom x)").parse().unwrap(),
            Term::Atom(Box::new(Term::Variable("x".to_string())))
        )
    }

    #[test]
    fn eq() {
        assert_eq!(
            create_parser("(eq x y)").parse().unwrap(),
            Term::Eq(
                Box::new(Term::Variable("x".to_string())),
                Box::new(Term::Variable("y".to_string()))
            )
        )
    }

    #[test]
    fn car() {
        assert_eq!(
            create_parser("(car x)").parse().unwrap(),
            Term::Car(Box::new(Term::Variable("x".to_string())))
        )
    }

    #[test]
    fn cdr() {
        assert_eq!(
            create_parser("(cdr x)").parse().unwrap(),
            Term::Cdr(Box::new(Term::Variable("x".to_string())))
        )
    }

    #[test]
    fn cons() {
        assert_eq!(
            create_parser("(cons x y)").parse().unwrap(),
            Term::Cons(
                Box::new(Term::Variable("x".to_string())),
                Box::new(Term::Variable("y".to_string()))
            )
        )
    }

    #[test]
    fn variable() {
        assert_eq!(
            create_parser("x").parse().unwrap(),
            Term::Variable("x".to_string())
        )
    }

    #[test]
    fn lambda() {
        assert_eq!(
            create_parser("(lambda (x) x)").parse().unwrap(),
            Term::Lambda(
                vec!["x".to_string()],
                Box::new(Term::Variable("x".to_string()))
            )
        );

        assert_eq!(
            create_parser("(lambda () x)").parse().unwrap(),
            Term::Lambda(vec![], Box::new(Term::Variable("x".to_string())))
        );
    }

    #[test]
    fn apply() {
        assert_eq!(
            create_parser("(apply f (,x ,y ,z))").parse().unwrap(),
            Term::Apply(
                Box::new(Term::Variable("f".to_string())),
                vec![
                    Box::new(Term::Variable("x".to_string())),
                    Box::new(Term::Variable("y".to_string())),
                    Box::new(Term::Variable("z".to_string()))
                ]
            )
        );

        assert_eq!(
            create_parser("(apply f (,x))").parse().unwrap(),
            Term::Apply(
                Box::new(Term::Variable("f".to_string())),
                vec![Box::new(Term::Variable("x".to_string())),]
            )
        );

        assert_eq!(
            create_parser("(apply f ())").parse().unwrap(),
            Term::Apply(Box::new(Term::Variable("f".to_string())), vec![])
        );
    }

    #[test]
    fn label() {
        assert_eq!(
            create_parser("(label f x)").parse().unwrap(),
            Term::Label("f".to_string(), Box::new(Term::Variable("x".to_string())))
        )
    }

    #[test]
    fn quote() {
        assert_eq!(
            create_parser("'x").parse().unwrap(),
            Term::Quote(SExpression::Symbol("x".to_string()))
        );

        assert_eq!(
            create_parser("'(x y)").parse().unwrap(),
            Term::Quote(SExpression::Pair(
                Box::new(SExpression::Symbol("x".to_string())),
                Box::new(SExpression::Symbol("y".to_string()),)
            ))
        );
    }
}
