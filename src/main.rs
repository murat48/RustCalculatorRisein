enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn main() {
    println!("Calculator");

    let first_number = 15.0;
    let operation = "-";
    let second_number = 5.0;

    let operation_enum = match operation {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    let result = calculate(operation_enum);
    println!("The result is: {}", result);
}
fn calculate(result: Operation) -> f64 {
    match result {
        Operation::Add(s1, s2) => {
            let sum: f64 = s1 + s2;
            println!("Result is add {}", sum);
            return sum;
        }
        Operation::Divide(s1, s2) => {
            if s2 != 0.0 {
                let rslt: f64 = s1 / s2;
                println!("Result is divide {}", rslt);
                return rslt;
            } else {
                panic!("Cannot divide by zero!");
            }
        }
        Operation::Multiply(s1, s2) => {
            let rslt: f64 = s1 * s2;
            println!("Result is multiply {}", rslt);
            return rslt;
        }
        Operation::Subtract(s1, s2) => {
            let rslt: f64 = s1 - s2;
            println!("Result is subtract {}", rslt);
            return rslt;
        }
    }
}
