use std::io;

pub fn get_user_input() -> String {
    let mut expression = String::new();

    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read the line");

    expression.trim().to_string()
}
