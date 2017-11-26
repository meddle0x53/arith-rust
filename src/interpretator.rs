use lexer;
use parser;
use parser::Term;
use std::fmt;

#[derive(PartialEq)]
pub struct EvalError {
    message: String
}

impl fmt::Debug for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EvalError : {}", self.message)
    }
}

macro_rules! eval_error {
    ($($arg:tt)*) => (
        return Err(EvalError { message: format!($($arg)*) })
    )
}

pub fn run(source: &str) -> Result<String, String> {
    let tokens = match lexer::tokenize(source) {
        Ok(res) => res,
        Err(lexer::SyntaxError{message: msg, ..}) => return Err(msg)
    };
    let terms = match parser::parse(&tokens) {
        Ok(res) => res,
        Err(parser::ParseError{message: msg}) => return Err(msg)
    };
    let values = terms.iter()
        .map(|t| eval(t.clone()))
        .map(|t| t.to_string())
        .fold("".to_string(), |acc, v| if acc.is_empty() {
            v
        } else {
            format!("{}\n{}", acc, v)
        });

    Ok(values)
}

fn eval(t: Term) -> Term {
    match eval1(t.clone()) {
        Ok(t1) => eval(t1),
        Err(EvalError{ message: _ }) => t
    }
}

fn is_numeric_val(t: &Term) -> bool {
    match t {
        &Term::Zero => true,
        &Term::Succ(ref t1) => is_numeric_val(t1),
        _ => false
    }
}

fn eval1(t: Term) -> Result<Term, EvalError> {
    match t {
        Term::If(box Term::True, t2, _) => Ok(*t2),
        Term::If(box Term::False, _, t3) => Ok(*t3),
        Term::If(t1, t2, t3) => Ok(Term::If(Box::new(eval1(*t1)?), t2, t3)),
        Term::Succ(t1) => Ok(Term::Succ(Box::new(eval1(*t1)?))),
        Term::Pred(box Term::Zero) => Ok(Term::Zero),
        Term::Pred(box Term::Succ(box ref nv)) if is_numeric_val(nv) => Ok((*nv).clone()),
        Term::Pred(t1) => Ok(Term::Pred(Box::new(eval1(*t1)?))),
        Term::IsZero(box Term::Zero) => Ok(Term::True),
        Term::IsZero(box Term::Succ(box ref nv)) if is_numeric_val(nv) => Ok(Term::False),
        Term::IsZero(box t1) => Ok(Term::IsZero(Box::new(eval1(t1)?))),
        t1 => eval_error!("No rule applies for {:?}", t1)
    }
}

#[test]
fn interpret_is_zero_test() {
    assert_eq!(run("is_zero zero").unwrap(), "true");
    assert_eq!(run("is_zero succ zero").unwrap(), "false");
    assert_eq!(run("is_zero true").unwrap(), "is_zero true");
}

#[test]
fn interpret_numbers_test() {
    assert_eq!(run("zero").unwrap(), "0");
    assert_eq!(run("succ zero").unwrap(), "1");
    assert_eq!(run("succ pred succ zero").unwrap(), "1");
    assert_eq!(run("succ zero pred succ zero").unwrap(), "1\n0");
}

#[test]
fn zero_is_numeric_val_test() {
    assert!(is_numeric_val(&Term::Zero));
}

#[test]
fn succ_num_is_numeric_val_test() {
    assert!(is_numeric_val(&Term::Succ(Box::new(Term::Zero))));
}

#[test]
fn non_num_is_numeric_val_test() {
    assert!(!is_numeric_val(&Term::True));
    assert!(!is_numeric_val(&Term::False));
}

#[test]
fn eval_numeric_value_test() {
    assert_eq!(
        eval(
            Term::If(
                box Term::False,
                box Term::Zero,
                box Term::Pred(box Term::Succ(box Term::Zero))
            )
        ),
        Term::Zero
    );
}

#[test]
fn eval_if_zero_value_test() {
    assert_eq!(
        eval(
            Term::If(
                box Term::IsZero(
                    box Term::Pred(box Term::Succ(box Term::Zero))
                ),
                box Term::Zero,
                box Term::Succ(box Term::Zero)
            )
        ),
        Term::Zero
    );
}

#[test]
fn eval_is_zero_if_test() {
    assert_eq!(
        eval(
            Term::IsZero(
                box Term::If(
                    box Term::IsZero(
                        box Term::Pred(box Term::Succ(box Term::Zero))
                    ),
                    box Term::Zero,
                    box Term::Succ(box Term::Zero)
                )
            )
        ),
        Term::True
    );
}

#[test]
fn if_true_eval1_test() {
    assert_eq!(
        eval1(
            Term::If(
                box Term::True,
                box Term::Zero,
                box Term::Succ(box Term::Zero)
            )
        ).ok().unwrap(),
        Term::Zero
    );
}

#[test]
fn if_false_eval1_test() {
    let one = Term::Succ(box Term::Zero);
    assert_eq!(
        eval1(
            Term::If(box Term::False, box Term::Zero, box one.clone())
        ).ok().unwrap(),
        one
    );
}

#[test]
fn if_complex_eval1_test() {
    let one = Term::Succ(box Term::Zero);
    let inner_if = Term::If(box Term::True, box Term::False, box Term::True);
    assert_eq!(
        eval1(
            Term::If(box inner_if, box Term::Zero, box one.clone())
        ).ok().unwrap(),
        Term::If(box Term::False, box Term::Zero, box one)
    );
}

#[test]
fn succ_eval1_test() {
    let one = Term::Succ(box Term::Zero);
    let two = Term::Succ(box one.clone());
    let three = Term::Succ(box two);
    let four = Term::Succ(box three);
    let five = Term::Succ(box four);

    assert_eq!(
        eval1(five).err().unwrap(),
        EvalError{ message: "No rule applies for Zero".to_string() }
    );
}

#[test]
fn pred_zero_eval1_test() {
    assert_eq!(
        eval1(Term::Pred(box Term::Zero)).ok().unwrap(),
        Term::Zero
    );
}

#[test]
fn pred_succ_eval1_test() {
    assert_eq!(
        eval1(Term::Pred(box Term::Succ(box Term::Zero))).ok().unwrap(),
        Term::Zero
    );
}

#[test]
fn pred_eval1_test() {
    assert_eq!(
        eval1(
            Term::Pred(
                box Term::If(
                    box Term::True,
                    box Term::Succ(box Term::Zero),
                    box Term::False
                )
            )
        ).ok().unwrap(),
        Term::Pred(box Term::Succ(box Term::Zero))
    );
}

#[test]
fn is_zero_zero_eval1_test() {
    assert_eq!(
        eval1(Term::IsZero(box Term::Zero)).ok().unwrap(),
        Term::True
    );
}

#[test]
fn is_zero_succ_eval1_test() {
    assert_eq!(
        eval1(Term::IsZero(box Term::Succ(box Term::Zero))).ok().unwrap(),
        Term::False
    );
}

#[test]
fn is_zero_eval1_test() {
    assert_eq!(
        eval1(Term::IsZero(box Term::Pred(box Term::Zero))).ok().unwrap(),
        Term::IsZero(box Term::Zero)
    );
}
