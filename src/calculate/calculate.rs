use crate::allowed_operators::Operator;
use crate::utils::{find_operator, parse_number};

pub fn calculate_expression(expression: &String) -> Option<i32> {
    let operator = find_operator(&expression);

    match operator {
        Some(operator) => perform_calculation(expression, operator),
        None => None,
    }
}

fn perform_calculation(expression: &String, operator: Operator) -> Option<i32> {
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
