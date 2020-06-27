mod err;

use err::*;
use std::{io, str::Chars};
use io::{Write, stdout};

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
    pub fn new(input: &str) -> Result<Operand, CalcError> {
        let numtest = input.parse::<f64>().is_ok();
        if numtest == true {
            let val: f64 = input.parse::<f64>().unwrap();
            Ok(Operand::Value(val))
        } else {
            let trimmed_input = &input[1..(input.len()-1)];

            let exp = Expression::new(trimmed_input)?;
            Ok(Operand::Compound(Box::new(exp)))
        }
    }

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
    pub fn new(input: &str) -> Result<Expression, CalcError> {
        let input = input.trim();
        let op = Expression::get_operator(input)?;

        let mut iter = input[1..].chars();
        let (left, right) = Expression::get_operands(&mut iter)?;
        
        Ok(Expression {
            operator: op,
            l_expr: left,
            r_expr: right,
        })
    }

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
                    Some(operator_char.to_string()),
                ))
            }
        };
        Ok(result)
    }

    fn get_operands(input: &mut Chars) -> Result<(Box<Operand>, Box<Operand>), CalcError> {
        let mut acc: Vec<Vec<char>> = Vec::new();
        let mut parse_state = ParseState::Waiting;

        for c in input {
            let parse_result = match parse_state {
                ParseState::Waiting => Expression::handle_waiting(c),
                ParseState::Number => Expression::handle_num(c),
                ParseState::Expression(count) => Expression::handle_exp(c, count),
            };
            
            let (new_state, maybe_value) = parse_result?;
            match parse_state {
                // If the parser was in the Waiting state and a value is found,
                // Create a new operand and put it on the accumulator.
                // Otherwise, do nothing
                // Either way, advanced the parse_state afterwards
                ParseState::Waiting => {
                    match maybe_value {
                        Some(c) => {
                            let new_operand = vec![c];
                            acc.push(new_operand);
                        },
                        None => {
                            // no-op
                        },
                    }
                    parse_state = new_state;
                },
                // In any non-waiting state, put whatever token was found onto the
                // last (most recent) operand in the accumulator
                // Then advance the parse_state
                _ => {
                    match maybe_value {
                        Some(c) => {
                            acc.last_mut().unwrap().push(c)
                        },
                        None => {
                            // no-op
                        }
                    }
                    parse_state = new_state;
                }
            }
        }

        let raw_op1: String = match acc.get(0) {
            Some(op) => op,
            None => return Err(CalcError::new::<&str>(ErrorCategory::SyntaxError, "Operand missing", None)),
        }.into_iter().collect();
        
        let raw_op2: String = match acc.get(1) {
            Some(op) => op,
            None => return Err(CalcError::new::<&str>(ErrorCategory::SyntaxError, "Operand missing", None)),
        }.into_iter().collect();

        let op1 = Box::new(Operand::new(&raw_op1[..])?);
        let op2 = Box::new(Operand::new(&raw_op2[..])?);
        
        Ok((op1, op2))
    }

    fn handle_waiting(input_char: char) -> Result<(ParseState, Option<char>), CalcError> {
        if input_char.is_whitespace() {
            return Ok((ParseState::Waiting, None));
        } else if input_char == '-' || input_char.is_numeric() {
            let return_val = (ParseState::Number, Some(input_char.clone()));
            return Ok(return_val);
        } else if input_char == '(' {
            let return_val = (ParseState::Expression(0), Some(input_char.clone()));
            return Ok(return_val);
        } else {
            return Err(CalcError::new(
                ErrorCategory::SyntaxError,
                "Bad token for state: 'Waiting'",
                Some(input_char.to_string()),
            ));
        }
    }

    fn handle_num(input_char: char) -> Result<(ParseState, Option<char>), CalcError> {
        if input_char == '.' || input_char.is_numeric() {
            let return_val = (ParseState::Number, Some(input_char.clone()));
            return Ok(return_val);
        } else if input_char.is_whitespace() {
            return Ok((ParseState::Waiting, None));
        } else {
            return Err(CalcError::new(
                ErrorCategory::SyntaxError,
                "Bad token for state: 'Number'",
                Some(input_char.to_string()),
            ));
        }
    }

    fn handle_exp(input_char: char, paren_count: i32) -> Result<(ParseState, Option<char>), CalcError> {
        if paren_count == 0 {
            if input_char == '(' {
                let return_val = (ParseState::Expression(paren_count+1), Some(input_char.clone()));
                return Ok(return_val)
            } else if input_char == ')' {
                let return_val = (ParseState::Waiting, Some(input_char.clone()));
                return Ok(return_val)
            } else {
                let return_val = (ParseState::Expression(paren_count), Some(input_char.clone()));
                return Ok(return_val)
            }
        } else {
            if input_char == '(' {
                let return_val = (ParseState::Expression(paren_count+1), Some(input_char.clone()));
                return Ok(return_val)
            } else if input_char == ')' {
                let return_val = (ParseState::Expression(paren_count-1), Some(input_char.clone()));
                return Ok(return_val)
            } else {
                let return_val = (ParseState::Expression(paren_count), Some(input_char.clone()));
                return Ok(return_val)
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

fn main() {
    let mut buffer = String::new();
    println!("Enter :q to quit");
    loop {
        let input = gather_exp(&mut buffer);
        if input == ":q" {
            println!("Quitting");
            break;
        } else {
            let expr = Expression::new(input);
            match expr {
                Ok(valid) => println!("= {:#?}", valid.calculate()),
                Err(e) => println!("Expression was invalid! {:#?}", { e }),
            }
            buffer.clear();
        }
    }
}

fn gather_exp(input: &mut String) -> &str {
    print!("> ");
    let _ = stdout().flush();
    io::stdin()
        .read_line(input)
        .expect("Fatal Error: Failed to read expression!");
    input.trim()
}

fn get_chr(index: usize, source: &str) -> Result<char, CalcError> {
    match source.chars().nth(index) {
        Some(chr) => Ok(chr),
        None => Err(CalcError::new::<&str>(
            ErrorCategory::SyntaxError,
            "Issue grabbing character",
            None,
        )),
    }
}
