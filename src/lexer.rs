use crate::token::Token;
use regex::Regex;

pub struct Lexer {
    buf: Box<dyn Iterator<Item = String>>,
}

impl Lexer {
    pub fn new(s: &String) -> Self {
        let re = Regex::new(
            format!(
                r"'|label|apply|lambda|cons|cdr|car|eq|atom|cond|,|\(|\)|{}|{}|.*",
                Self::ident_regex(),
                Self::space_regex()
            )
            .as_str(),
        )
        .unwrap();

        Lexer {
            buf: Box::new(
                re.find_iter(s.as_str())
                    .map(|m| m.as_str().to_string())
                    // NOTE: Create a vector to avoid an error about `re` lifetime
                    .collect::<Vec<String>>()
                    .into_iter(),
            ),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let Some(s) = self.buf.next() else {
            return None;
        };

        match s.as_str() {
            "'" => Some(Token::Quote),
            "label" => Some(Token::Label),
            "apply" => Some(Token::Apply),
            "lambda" => Some(Token::Lambda),
            "cons" => Some(Token::Cons),
            "cdr" => Some(Token::Cdr),
            "car" => Some(Token::Car),
            "eq" => Some(Token::Eq),
            "atom" => Some(Token::Atom),
            "cond" => Some(Token::Cond),
            "," => Some(Token::Comma),
            ")" => Some(Token::RParen),
            "(" => Some(Token::LParen),
            s => {
                if Regex::new(&Self::ident_regex()).unwrap().is_match(s) {
                    Some(Token::Id(s.to_string()))
                } else if Regex::new(&Self::space_regex()).unwrap().is_match(s) {
                    self.next_token()
                } else {
                    None
                }
            }
        }
    }

    fn ident_regex() -> &'static str {
        r"[a-zA-Z][a-zA-Z0-9]*"
    }

    fn space_regex() -> &'static str {
        r"\s+"
    }
}
