use std::io;

enum Operator {
    Plus(usize),
    Minus(usize),
    Multiply(usize),
    Divide(usize),
}

impl Operator {
    fn get_position(&self) -> usize {
        match self {
            Operator::Plus(n)
            | Operator::Minus(n)
            | Operator::Multiply(n)
            | Operator::Divide(n) => *n,
        }
    }
}

fn main() {
    input_prompt();
    let expression = enter_expression();
    let result = calculate_expression(&expression);
    show_result(&expression, result);
}

fn show_result(expression: &String, result: Option<i32>) {
    match result {
        Some(result) => println!("\n{expression} = {result}"),
        None => println!("\nUnexpected error when evaluating an expression"),
    }
}

fn calculate_expression(expression: &String) -> Option<i32> {
    let operator = find_operator(&expression);

    match operator {
        Some(operator) => {
            let first_number = parse_number(&expression[0..operator.get_position()]);
            let second_number = parse_number(&expression[operator.get_position() + 1..]);

            match (first_number, second_number) {
                (Some(first_number), Some(second_number)) => match operator {
                    Operator::Plus(_) => Some(first_number + second_number),
                    Operator::Minus(_) => Some(first_number - second_number),
                    Operator::Multiply(_) => Some(first_number * second_number),
                    Operator::Divide(_) => {
                        if second_number != 0 {
                            return Some(first_number / second_number);
                        }
                        println!("Division by zero is not allowed");
                        None
                    }
                },
                _ => None,
            }
        }
        None => None,
    }
}

fn find_operator(expression: &String) -> Option<Operator> {
    for (i, &item) in expression.as_bytes().iter().enumerate() {
        match item {
            b'+' => return Some(Operator::Plus(i)),
            b'-' => return Some(Operator::Minus(i)),
            b'*' => return Some(Operator::Multiply(i)),
            b'/' => return Some(Operator::Divide(i)),
            _ => {}
        };
    }
    None
}

fn parse_number(number_to_parse: &str) -> Option<i32> {
    match number_to_parse.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Error due parse a number");
            None
        }
    }
}

fn enter_expression() -> String {
    let mut expression = String::new();

    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read the line");

    expression.trim().to_string()
}

fn input_prompt() {
    println!("Write an expression using one of the following operators: +, -, /, *");
    println!("For example, you can write an expression in such form: 2 + 2");
}
