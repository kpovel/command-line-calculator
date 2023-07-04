mod console_output;
mod calculate;
mod input;
mod allowed_operators;
mod utils;

use crate::console_output::results;

fn main() {
    console_output::input_prompt();
    let expression = input::get_user_input();
    let result = calculate::calculate_expression(&expression);
    results::show_result(&expression, result);
}

