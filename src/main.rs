use std::io;

struct Operator {
    operator_position: usize,
    operator: Operators,
}

enum Operators {
    Plus,
    Minus,
    Multiply,
    Divide,
}

fn main() {
    input_prompt();
    let expression = enter_expression();
    let result = calculate_expression(&expression);
    show_result(&expression, result);
}

fn show_result(expression: &String, result: Option<i32>) {
    match result {
        Some(result)=> println!("\n{expression} = {result}"),
        None => println!("\nUnexpected error when evaluating an expression")
    }
}

fn calculate_expression(expression: &String) -> Option<i32> {
    let operator = find_operator(&expression);

    match operator {
        Some(operator) => {
            let first_number = parse_number(&expression[0..operator.operator_position ]);
            let second_number = parse_number(&expression[operator.operator_position + 1..]);
            dbg!(&first_number);
            dbg!(&second_number);

            match operator.operator {
                Operators::Plus => Some(first_number + second_number),
                Operators::Minus => Some(first_number - second_number),
                Operators::Multiply => Some(first_number * second_number),
                Operators::Divide => Some(first_number / second_number),
            }
        }
        None => None,
    }
}

fn find_operator(expression: &String) -> Option<Operator> {
    let bytes_expression = expression.as_bytes();
    for (i, &item) in bytes_expression.iter().enumerate() {
        if item == b'+' {
            return Some(Operator {
                operator_position: i,
                operator: Operators::Plus,
            });
        } else if item == b'-' {
            return Some(Operator {
                operator_position: i,
                operator: Operators::Minus,
            });
        } else if item == b'*' {
            return Some(Operator {
                operator_position: i,
                operator: Operators::Multiply,
            });
        } else if item == b'/' {
            return Some(Operator {
                operator_position: i,
                operator: Operators::Divide,
            });
        }
    }
    return None;
}

fn parse_number(number_to_parse: &str) -> i32 {
    match number_to_parse.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error to parse a number");
            0
        }
    }
}

fn enter_expression() -> String {
    let mut expression = String::new();

    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read the line");

    let expression = expression.trim().to_string();

    return expression;
}

fn input_prompt() {
    println!("Write an expression using one of the following operators: +, -, /, *");
    println!("For example, you can write an expression in such form: 2 + 2");
}
