use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Succ,
    Pred,
    Zero,
    True,
    False,
    If,
    Then,
    Else,
    IsZero
}

use self::Token::*;

pub struct SyntaxError {
    pub message: String,
    position: usize
}

impl fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SyntaxError: {} (position: {})", self.message, self.position)
    }
}

macro_rules! syntax_error {
    ($position:ident, $($arg:tt)*) => (
        return Err(SyntaxError { message: format!($($arg)*), position: $position + 1 })
    )
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, SyntaxError> {
    let mut chars = source.chars().enumerate().peekable();
    let mut tokens = Vec::new();

    while let Some((i, c)) = chars.next() {
        match c {
            'a'...'z' | 'A'...'Z' => {
                let mut word = c.to_string();
                while let Some((j, d)) = chars.next() {
                    match d {
                        'a'...'z' | 'A'...'Z' | '_' => word.push(d),
                        _ if d.is_whitespace() => break,
                        _ => {
                            syntax_error!(j, "Invalid character : {}", d)
                        }
                    }
                }

                let token = match word.as_ref() {
                    "succ"    => Succ,
                    "pred"    => Pred,
                    "zero"    => Zero,
                    "true"    => True,
                    "false"   => False,
                    "is_zero" => IsZero,
                    "if"      => If,
                    "then"    => Then,
                    "else"    => Else,
                    _ => syntax_error!(i, "Invalid keyword : {}", word)
                };
                tokens.push(token);
            },
            _ => {
                if c.is_whitespace() {
                    ()
                } else {
                    syntax_error!(i, "Invalid character : {}", c)
                }
            }
        }
    }

    Ok(tokens)
}

#[test]
fn tokenize_zero_test() {
    assert_eq!(
        tokenize("zero").ok().unwrap(),
        vec![Zero]
    );
}

#[test]
fn tokenize_one_test() {
    assert_eq!(
        tokenize("succ zero").ok().unwrap(),
        vec![Succ, Zero]
    );
}

#[test]
fn tokenize_five_test() {
    assert_eq!(
        tokenize("succ succ succ succ succ zero").ok().unwrap(),
        vec![Succ, Succ, Succ, Succ, Succ, Zero]
    );
}

#[test]
fn tokenize_is_zero_zero_test() {
    assert_eq!(
        tokenize("is_zero zero").ok().unwrap(),
        vec![IsZero, Zero]
    );
}

#[test]
fn tokenize_is_zero_one_test() {
    assert_eq!(
        tokenize("is_zero succ zero").ok().unwrap(),
        vec![IsZero, Succ, Zero]
    );
}
