use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b == 0.0 {
                println!("Error: You cannot divide a number by zero!");
                std::process::exit(1);
            }
            a / b
        }
    }
}

fn main() {
    println!("Enter the first number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Girdi alınamadı");
    let num1: f64 = input1.trim().parse().expect("Geçersiz sayı girdiniz");

    println!("Choose process (+, -, *, /):");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Girdi alınamadı");
    let operator = operator.trim();

    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Girdi alınamadı");
    let num2: f64 = input2.trim().parse().expect("Geçersiz sayı girdiniz");

    let operation = match operator {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => {
            println!("Invalid process!");
            std::process::exit(1);
        }
    };

    let result = calculate(operation);
    println!("Result: {}", result);
}
