use crate::allowed_operators::Operator;

pub fn find_operator(expression: &String) -> Option<Operator> {
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

pub fn parse_number(number_to_parse: &str) -> Option<i32> {
    match number_to_parse.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Error due parse a number");
            None
        }
    }
}

