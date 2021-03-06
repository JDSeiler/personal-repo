mod err;

use err::*;
use std::{io, io::stdout, io::Write, str::Chars};
use std::cell::RefCell;

/*
To create a "global" variable in Rust you need two things:
1. Restriction of that variable to a single thread.
2. Wrapping of the value in a RefCell to allow dynamic borrow-checking.

static ensures that this variable is always a pointer to the same RefCell,
even if the contents of the RC change.
*/
thread_local!(static LAST_RESULT: RefCell<f64> = RefCell::new(0.0));

#[derive(Debug)]
enum ParseState {
    Waiting,
    Number,
    Expression(i32),
}

#[derive(Debug)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug)]
enum Operand {
    Value(f64),
    Compound(Box<Expression>),
}

impl Operand {
    /**
    Creates a new operand by either:
    1. Parsing it directly to a float
    2. Parsing it to a float via `p` which represents the previous calculation's result.
    3. Recursively parsing it as an Expression
    */
    pub fn new(input: &str) -> Result<Operand, CalcError> {
        if let Ok(val) = input.parse::<f64>() {
            Ok(Operand::Value(val))
        } else if input == "p" {
          Ok(Operand::Value(LAST_RESULT.with(|cell| cell.replace_with(|old| old.clone()))))
        } else {
            let input_no_parens = &input[1..(input.len() - 1)];

            let exp = Expression::new(input_no_parens)?;
            Ok(Operand::Compound(Box::new(exp)))
        }
    }

    /**
    Returns the value of an operand by either:
    1. Returning its value (if it's just a f64)
    2. Calculating the value of itself recursively (if it's an expression)
    */
    fn evaluate(&self) -> f64 {
        match self {
            Operand::Value(num) => *num,
            Operand::Compound(expr) => expr.calculate(),
        }
    }
}

#[derive(Debug)]
struct Expression {
    operator: Operator,
    l_expr: Box<Operand>,
    r_expr: Box<Operand>,
}

impl Expression {
    /**
    Recursively generates an Expression, which is effectively a binary tree
    representing the arithmetic expression passed as input.
    */
    pub fn new(input: &str) -> Result<Expression, CalcError> {
        let input = input.trim();
        let op = Expression::get_operator(input)?;

        let (left, right) = Expression::get_operands(input[1..].chars())?;

        Ok(Expression {
            operator: op,
            l_expr: left,
            r_expr: right,
        })
    }

    /**
    Finds the operator of an expression by fetching the first character
    and pattern matching it. If the character cannot be retrieved or is
    not a valid operator, this function throws an error.
    */
    fn get_operator(input: &str) -> Result<Operator, CalcError> {
        let operator_char = get_chr(0, input)?;
        let result = match operator_char {
            '+' => Operator::Addition,
            '-' => Operator::Subtraction,
            '*' => Operator::Multiplication,
            '/' => Operator::Division,
            _ => {
                return Err(CalcError::new(
                    ErrorCategory::UnkownOperatorError,
                    "Operator not recognized",
                    Some(operator_char),
                ))
            }
        };
        Ok(result)
    }

    /**
    Uses state-based parsing to extract the operands from an expression.

    The processing works by iterating over every character in the input and:
    1. Matching on the current parse_state and handing the current character to the
    appropriate function. These functions return the next state of the parser and may
    return a character that represents the next Operand char. This character may be None
    if the input_char is, for instance, whitespace.
    2. Matching on the parse state again to decide whether this character is the start
    of a new operand or if it should be appended to the most recent operand. This in theory
    allows the program to parse arbitrarily many operands but this is not currently supported.
    3. Advance the parse state to the new state produced in #1.
    */
    fn get_operands(input: Chars) -> Result<(Box<Operand>, Box<Operand>), CalcError> {
        let mut acc: Vec<String> = Vec::new();
        let mut parse_state = ParseState::Waiting;

        for c in input {
            let (new_state, maybe_value) = match parse_state {
                ParseState::Waiting => Expression::handle_waiting(c),
                ParseState::Number => Expression::handle_num(c),
                ParseState::Expression(count) => Expression::handle_exp(c, count),
            }?;

            match parse_state {
                // If the parser was in the Waiting state and a value is found,
                // Create a new operand and put it on the accumulator.
                // Otherwise, do nothing
                // Either way, advanced the parse_state afterwards
                ParseState::Waiting => {
                    if let Some(c) = maybe_value {
                        acc.push(String::from(c.to_string()));
                    }
                }
                // In any non-waiting state, put whatever token was found onto the
                // last (most recent) operand in the accumulator
                // Then advance the parse_state
                _ => {
                    if let Some(c) = maybe_value {
                        // Strings are backed by Vec<char> so this is a fast operation.
                        acc.last_mut().unwrap().push(c)
                    }
                }
            }
            parse_state = new_state;
        }

        let raw_op1: String = acc.get(0).ok_or_else(|| CalcError::new(
                    ErrorCategory::SyntaxError,
                    "Operand missing",
                    None))?.to_string();
        
        let raw_op2: String = acc.get(1).ok_or_else(|| CalcError::new(
                    ErrorCategory::SyntaxError,
                    "Operand missing",
                    None))?.to_string();

        let op1 = Box::new(Operand::new(&raw_op1[..])?);
        let op2 = Box::new(Operand::new(&raw_op2[..])?);

        Ok((op1, op2))
    }

    fn handle_waiting(input_char: char) -> Result<(ParseState, Option<char>), CalcError> {
        if input_char.is_whitespace() {
            return Ok((ParseState::Waiting, None));
        } else if input_char == '-' || input_char.is_numeric() || input_char == 'p' {
            let return_val = (ParseState::Number, Some(input_char));
            return Ok(return_val);
        } else if input_char == '(' {
            let return_val = (ParseState::Expression(0), Some(input_char));
            return Ok(return_val);
        } else {
            return Err(CalcError::new(
                ErrorCategory::SyntaxError,
                "Bad token for state: 'Waiting'",
                Some(input_char),
            ));
        }
    }

    fn handle_num(input_char: char) -> Result<(ParseState, Option<char>), CalcError> {
        if input_char == '.' || input_char.is_numeric() {
            let return_val = (ParseState::Number, Some(input_char));
            return Ok(return_val);
        } else if input_char.is_whitespace() {
            return Ok((ParseState::Waiting, None));
        } else {
            return Err(CalcError::new(
                ErrorCategory::SyntaxError,
                "Bad token for state: 'Number'",
                Some(input_char),
            ));
        }
    }

    fn handle_exp(
        input_char: char,
        paren_count: i32,
    ) -> Result<(ParseState, Option<char>), CalcError> {
        if paren_count == 0 {
            if input_char == '(' {
                let return_val = (
                    ParseState::Expression(paren_count + 1),
                    Some(input_char),
                );
                return Ok(return_val);
            } else if input_char == ')' {
                let return_val = (ParseState::Waiting, Some(input_char));
                return Ok(return_val);
            } else {
                let return_val = (
                    ParseState::Expression(paren_count),
                    Some(input_char),
                );
                return Ok(return_val);
            }
        } else {
            if input_char == '(' {
                let return_val = (
                    ParseState::Expression(paren_count + 1),
                    Some(input_char),
                );
                return Ok(return_val);
            } else if input_char == ')' {
                let return_val = (
                    ParseState::Expression(paren_count - 1),
                    Some(input_char),
                );
                return Ok(return_val);
            } else {
                let return_val = (
                    ParseState::Expression(paren_count),
                    Some(input_char),
                );
                return Ok(return_val);
            }
        }
    }

    pub fn calculate(&self) -> f64 {
        match self.operator {
            Operator::Addition => self.l_expr.evaluate() + self.r_expr.evaluate(),
            Operator::Subtraction => self.l_expr.evaluate() - self.r_expr.evaluate(),
            Operator::Multiplication => self.l_expr.evaluate() * self.r_expr.evaluate(),
            Operator::Division => self.l_expr.evaluate() / self.r_expr.evaluate(),
        }
    }
}

fn set_last_result(new_value: Option<f64>) {
    let val = new_value.unwrap_or_default();
    // Because of the thread safety constrains required to use a static RefCell, LAST_RESULT
    // is actually a wrapper type. Calling `with` is required to get access to the RC inside.
    LAST_RESULT.with(|cell| cell.replace(val));
}

fn main() {
    let mut buffer = String::new();
    println!("Enter :q to quit");
    println!("Enter :c to reset the stored value of the last calculation.");
    println!("Enter p in the place of any number to insert the result of the previous calculation:");
    loop {
        buffer.clear();
        let input = gather_exp(&mut buffer);
        if input == ":q" {
            println!("Quitting");
            break;
        } else if input == ":c" {
            set_last_result(None);
        } else {
            let expr = Expression::new(input);
            match expr {
                Ok(valid) => {
                    let result = valid.calculate();
                    if result.is_infinite() {
                        println!("Cannot divide by 0");
                        set_last_result(None);
                    } else {
                        println!("= {}", result);
                        set_last_result(Some(result));
                    }
                }
                Err(e) => { 
                    println!("Expression was invalid! {}", { e });
                    set_last_result(None);
                }
            }
        }
    }
}

fn gather_exp(input: &mut String) -> &str {
    print!("> ");
    let _ = stdout().flush().unwrap();
    io::stdin()
        .read_line(input)
        .unwrap();
    input.trim()
}

fn get_chr(index: usize, source: &str) -> Result<char, CalcError> {
    match source.chars().nth(index) {
        Some(chr) => Ok(chr),
        None => Err(CalcError::new(
            ErrorCategory::SyntaxError,
            "Issue grabbing character",
            None,
        )),
    }
}
