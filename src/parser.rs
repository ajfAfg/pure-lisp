use crate::{
    lexer::Lexer,
    syntax::{SExpression, Term},
    token::Token,
};

pub struct Parser(Lexer);

impl Parser {
    pub fn new(lex: Lexer) -> Self {
        Parser(lex)
    }

    pub fn parse(&mut self) -> Option<Term> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Option<Term> {
        let Some(token) = self.0.next_token() else {
            return None;
        };

        match token {
            Token::LParen => {
                let token = self.0.next_token()?;

                match token {
                    Token::Cond => {
                        let mut clauses = vec![];
                        loop {
                            let token = self.0.next_token()?;

                            match token {
                                Token::LParen => {
                                    let term1 = self.parse_term()?;
                                    let term2 = self.parse_term()?;

                                    if self.0.next_token() == Some(Token::RParen) {
                                        clauses.push((Box::new(term1), Box::new(term2)));
                                    } else {
                                        return None;
                                    }
                                }
                                Token::RParen => {
                                    return Some(Term::Cond(clauses));
                                }
                                _ => {
                                    return None;
                                }
                            }
                        }
                    }
                    Token::Atom => {
                        let term1 = self.parse_term()?;
                        match self.0.next_token()? {
                            Token::RParen => Some(Term::Atom(Box::new(term1))),
                            _ => None,
                        }
                    }
                    Token::Eq => {
                        let term1 = self.parse_term()?;
                        let term2 = self.parse_term()?;
                        match self.0.next_token()? {
                            Token::RParen => Some(Term::Eq(Box::new(term1), Box::new(term2))),
                            _ => None,
                        }
                    }
                    Token::Car => {
                        let term = self.parse_term()?;
                        match self.0.next_token()? {
                            Token::RParen => Some(Term::Car(Box::new(term))),
                            _ => None,
                        }
                    }
                    Token::Cdr => {
                        let term = self.parse_term()?;
                        match self.0.next_token()? {
                            Token::RParen => Some(Term::Cdr(Box::new(term))),
                            _ => None,
                        }
                    }
                    Token::Cons => {
                        let term1 = self.parse_term()?;
                        let term2 = self.parse_term()?;
                        match self.0.next_token()? {
                            Token::RParen => Some(Term::Cons(Box::new(term1), Box::new(term2))),
                            _ => None,
                        }
                    }
                    Token::Lambda => {
                        if self.0.next_token()? != Token::LParen {
                            return None;
                        }

                        let mut params = vec![];
                        loop {
                            let token = self.0.next_token()?;

                            match token {
                                Token::Id(id) => {
                                    params.push(id);
                                }
                                Token::RParen => {
                                    break;
                                }
                                _ => {
                                    return None;
                                }
                            }
                        }

                        let term = self.parse_term()?;
                        match self.0.next_token()? {
                            Token::RParen => Some(Term::Lambda(params, Box::new(term))),
                            _ => None,
                        }
                    }
                    Token::Apply => {
                        let term = self.parse_term()?;
                        let mut args = vec![];

                        if self.0.next_token()? != Token::LParen {
                            return None;
                        }

                        loop {
                            match self.0.next_token()? {
                                Token::Comma => (),
                                Token::RParen => {
                                    break;
                                }
                                _ => {
                                    return None;
                                }
                            };

                            args.push(Box::new(self.parse_term()?));
                        }

                        Some(Term::Apply(Box::new(term), args))
                    }
                    Token::Label => {
                        let token = self.0.next_token()?;
                        let term = self.parse_term()?;

                        match token {
                            Token::Id(id) if self.0.next_token()? == Token::RParen => {
                                Some(Term::Label(id, Box::new(term)))
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            Token::Id(id) => Some(Term::Variable(id)),
            Token::Quote => Some(Term::Quote(self.parse_sexpression()?)),
            _ => None,
        }
    }

    fn parse_sexpression(&mut self) -> Option<SExpression> {
        let Some(token) = self.0.next_token() else {
            return None;
        };

        match token {
            Token::Id(id) => Some(SExpression::Symbol(id)),
            Token::LParen => {
                let exp1 = self.parse_sexpression()?;
                let exp2 = self.parse_sexpression()?;
                match self.0.next_token() {
                    Some(Token::RParen) => Some(SExpression::Pair(Box::new(exp1), Box::new(exp2))),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
