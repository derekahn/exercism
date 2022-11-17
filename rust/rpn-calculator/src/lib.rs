#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;

    let mut stack: Vec<i32> = Vec::new();

    for input in inputs {
        match input {
            Value(number) => stack.push(*number),
            _ => match (stack.pop(), stack.pop()) {
                (Some(b), Some(a)) => match input {
                    Add => stack.push(a + b),
                    Subtract => stack.push(a - b),
                    Multiply => stack.push(a * b),
                    Divide => stack.push(a / b),
                    _ => return None,
                },
                _ => return None,
            },
        }
    }

    match stack.len() {
        1 => stack.pop(),
        _ => None,
    }
}
