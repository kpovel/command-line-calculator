pub fn show_result(expression: &String, result: Option<i32>) {
    match result {
        Some(result) => println!("\n{expression} = {result}"),
        None => println!("\nUnexpected error when evaluating an expression"),
    }
}
