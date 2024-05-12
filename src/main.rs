use std::io::{self, Write};

use pure_lisp::{environment::Environment, lexer::Lexer, parser::Parser, syntax::Value};

fn main() -> io::Result<()> {
    let mut env = Environment::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            // NOTE: Finish the program when the user presses Ctrl-D
            return Ok(());
        }

        match Parser::new(Lexer::new(&input.to_string())).parse() {
            Some(term) => match term.eval(&mut env) {
                Some(value) => println!("{}", to_string_value(&value)),
                None => println!("Semantics error"),
            },
            None => println!("Syntax error"),
        }
    }
}

fn to_string_value(value: &Value) -> String {
    match value {
        Value::Symbol(s) => s.clone(),
        Value::Pair(car, cdr) => format!("({} {})", to_string_value(car), to_string_value(cdr)),
        Value::Closure(_, _, _) => "<closure>".to_string(),
    }
}
