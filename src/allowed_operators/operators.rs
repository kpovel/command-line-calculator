pub enum Operator {
    Plus(usize),
    Minus(usize),
    Multiply(usize),
    Divide(usize),
}

impl Operator {
   pub fn get_position(&self) -> usize {
        match self {
            Operator::Plus(n)
            | Operator::Minus(n)
            | Operator::Multiply(n)
            | Operator::Divide(n) => *n,
        }
    }
}
