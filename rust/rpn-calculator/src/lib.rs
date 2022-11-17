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
            _ => {
                if stack.len() < 2 {
                    return None;
                }

                let b = stack.pop().unwrap_or(0);
                let a = stack.pop().unwrap_or(0);

                match input {
                    Add => stack.push(a + b),
                    Subtract => stack.push(a - b),
                    Multiply => stack.push(a * b),
                    Divide => stack.push(a / b),
                    _ => return None,
                }
            }
        }
    }

    match stack.len() {
        1 => stack.pop(),
        _ => None,
    }
}
