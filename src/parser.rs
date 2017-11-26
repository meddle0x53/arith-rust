use lexer::*;
use std::fmt;
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>),
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>)
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &True => write!(f, "true"),
            &False => write!(f, "false"),
            &If(ref t1, ref t2, ref t3) => write!(f, "if {} then {} else {}", t1, t2, t3),
            &Zero => write!(f, "0"),
            &Succ(ref t1) => {
                fn add(n: u32, t: &Term, f: &mut fmt::Formatter) -> fmt::Result {
                    match t {
                        &Zero => write!(f, "{}", n),
                        &Succ(ref s) => add(n + 1, s, f),
                        &Pred(ref s) => add(n - 1, s, f),
                        _ => write!(f, "(succ {})", t),
                    }
                }
                add(1, t1, f)
            }
            &Pred(ref t1) => write!(f, "pred {}", t1),
            &IsZero(ref t1) => write!(f, "is_zero {}", t1),
        }
    }
}

#[derive(PartialEq)]
pub struct ParseError {
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }
}
impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }
}

macro_rules! parse_error {
    ($($arg:tt)*) => (
        return Err(ParseError { message: format!($($arg)*)})
    )
}

use self::Term::*;

pub fn parse(tokens: &Vec<Token>) -> Result<Vec<Term>, ParseError> {
    let mut vec = Vec::new();
    let mut token_iterator: Iter<Token> = tokens.iter();

    while let Some(token) = token_iterator.next() {
        vec.push(parse_term_from_tokens(token, &mut token_iterator)?);
    }

    Ok(vec)
}

fn parse_term_from_tokens(token: &Token, tokens: &mut Iter<Token>) -> Result<Term, ParseError> {
    match token {
        &Token::True   => Ok(True),
        &Token::False  => Ok(False),
        &Token::Zero   => Ok(Zero),
        &Token::IsZero => Ok(IsZero(box parse_inner(tokens)?)),
        &Token::Succ   => Ok(Succ(box parse_inner(tokens)?)),
        &Token::Pred   => Ok(Pred(box parse_inner(tokens)?)),
        &Token::If     => {
            let condition = parse_inner(tokens)?;
            if Some(&Token::Then) == tokens.next() {
                let true_case = parse_inner(tokens)?;

                if Some(&Token::Else) == tokens.next() {
                    let false_case = parse_inner(tokens)?;

                    Ok(If(box condition, box true_case, box false_case))
                } else {
                    parse_error!("Invalid If statement. Missing 'then' clause.")
                }
            } else {
                parse_error!("Invalid If statement. Missing 'else' clause.")
            }
        },
        t => parse_error!("Unknown token : {:?}", t)
    }
}

fn parse_inner(tokens: &mut Iter<Token>) -> Result<Term, ParseError> {
    if let Some(next_token) = tokens.next() {
        parse_term_from_tokens(next_token, tokens)
    } else {
        parse_error!("Invalid program!")
    }
}

#[test]
fn parse_true_test() {
    assert_eq!(
        parse(&vec![Token::True]).ok().unwrap(),
        vec![True]
    );
}

#[test]
fn parse_false_test() {
    assert_eq!(
        parse(&vec![Token::False]).ok().unwrap(),
        vec![False]
    );
}

#[test]
fn parse_zero_test() {
    assert_eq!(
        parse(&vec![Token::Zero]).ok().unwrap(),
        vec![Zero]
    );
}

#[test]
fn parse_is_zero_test() {
    assert_eq!(
        parse(&vec![Token::IsZero, Token::Zero]).ok().unwrap(),
        vec![IsZero(box Zero)]
    );
}

#[test]
fn parse_is_zero_incomplete_test() {
    assert_eq!(
        parse(&vec![Token::IsZero]).err().unwrap(),
        ParseError{ message: "Invalid program!".to_string() }
    );
}

#[test]
fn parse_succ_test() {
    assert_eq!(
        parse(&vec![Token::Succ, Token::Succ, Token::Zero]).ok().unwrap(),
        vec![Succ(box Succ(box Zero))]
    );
}

#[test]
fn parse_pred_test() {
    assert_eq!(
        parse(&vec![Token::Pred, Token::Succ, Token::Succ, Token::Zero]).ok().unwrap(),
        vec![Pred(box Succ(box Succ(box Zero)))]
    );
}

#[test]
fn parse_simple_if_test() {
    assert_eq!(
        parse(&vec![Token::If, Token::True, Token::Then, Token::False, Token::Else, Token::True]).ok().unwrap(),
        vec![If(box True, box False, box True)]
    );
}
