use std::io;

fn main() {
    loop {
        let input = gather_exp();
        if input == "QQ" {
            break
        }
        else {
            let answer = evaluate(&input);
            println!("{}",answer);
        }
    }
}

fn evaluate(exp: &String) -> f64 {
    let numtest = exp.parse::<f64>().is_ok();
    if numtest == true {
        return exp.parse::<f64>().unwrap();
    }
    else {
        let parse1 = find_operand(&exp, 0).unwrap();
        let operand1 = parse1.0;
        let next_ind = parse1.1;
        let parse2 = find_operand(&exp, next_ind).unwrap();
        let operand2 = parse2.0;
        let operator = find_operator(&exp);
        match operator {
            Operator::Addition => evaluate(&operand1) + evaluate(&operand2),
            Operator::Subtraction => evaluate(&operand1) - evaluate(&operand2),
            Operator::Multiplication => evaluate(&operand1) * evaluate(&operand2),
            Operator::Division => evaluate(&operand1) / evaluate(&operand2),
            Operator::Null => panic!("Syntax Error!"),
        }
    }
}

fn gather_exp() -> String {
    // Funciton takes in input from std::io and returns a String
    // representing the expression

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Fatal Error: Failed to read expression!");
    return input.trim().to_owned();
}

fn find_operator(exp: &String) -> Operator {
    // Function takes in an expression and returns an enum
    // representing the operator type

    let op_char = exp.chars().nth(1).unwrap_or('0');
    match op_char {
        '+' => Operator::Addition,
        '-' => Operator::Subtraction,
        '*' => Operator::Multiplication,
        '/' => Operator::Division,
        _ => Operator::Null,
    }
}


fn find_operand(input: &String, index: usize) -> Option<(String, usize)> {
    let mut exp = input.clone();
    let len = exp.len();
    exp = exp[1..(len - 1)].to_string();
    for i in index..len {
        if (exp.chars().nth(i).unwrap() == '-') && (exp.chars().nth(i + 1).unwrap().is_digit(10)) {
            for digit_ind in i + 1..exp.len() {
                if exp.chars().nth(digit_ind).unwrap().is_digit(10) && (digit_ind == exp.len()-1) {
                    let num = &exp[i..=digit_ind];
                    return Some((num.to_string(), digit_ind));
                }
                else if exp.chars().nth(digit_ind).unwrap().is_digit(10) {
                    continue;
                }
                else {
                    let num = &exp[i..digit_ind];
                    return Some((num.to_string(), digit_ind));
                }
            }
        }
        if exp.chars().nth(i).unwrap().is_digit(10) {
            for digit_ind in i..exp.len() {
                if exp.chars().nth(digit_ind).unwrap().is_digit(10) && (digit_ind == exp.len() - 1)
                {
                    let num = &exp[i..=digit_ind];
                    return Some((num.to_string(), digit_ind));
                } else if exp.chars().nth(digit_ind).unwrap().is_digit(10) {
                    continue;
                } else {
                    let num = &exp[i..digit_ind];
                    return Some((num.to_string(), digit_ind));
                }
            }
        }
        if exp.chars().nth(i).unwrap() == '(' { 
            let mut open: u32 = 0;
            for exp_ind in i+1..exp.len() {
                if exp.chars().nth(exp_ind).unwrap() == '(' {
                    open = open + 1; 
                    continue;
                }
                if exp.chars().nth(exp_ind).unwrap() == ')' && (open > 0) {
                    open = open - 1; 
                    continue;
                }
                if exp.chars().nth(exp_ind).unwrap() == ')' && (open == 0) {
                    let expression = &exp[i..=exp_ind];
                    return Some((expression.to_string(), exp_ind));
                }

            }
        }
    }
    None
}

#[derive(PartialEq)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Null,
}

