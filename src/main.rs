use std::io;

fn main() {
    input_prompt();
    let expression = enter_expression();
    let result = calculate_expresstion(&expression);
    show_result(&expression, result);
}

fn show_result(expression: &String, result: i32) {
    println!("\n{expression} = {result}");
}

fn calculate_expresstion(expression: &String) -> i32 {
    let bytes_expression = expression.as_bytes();
    let mut operator_position: usize = 0;

    for (i, &item) in bytes_expression.iter().enumerate() {
        if item == b'+' || item == b'-' || item == b'/' || item == b'*' {
            operator_position = i;
        }
    }

    let first_number = parse_number(&expression[0..operator_position - 1]);
    let second_number = parse_number(&expression[operator_position + 1..]);

    match &expression[operator_position..operator_position + 1] {
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" => first_number * second_number,
        "/" => first_number / second_number,
        &_ => 0,
    }
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
        .expect("Faild to read the line");

    let expression = expression.trim().to_string();

    return expression;
}

fn input_prompt() {
    println!("Write an expression using one of the following operators: +, -, /, *");
    println!("For example, you can write an expression in such form: 2 + 2");
}
