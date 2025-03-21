// Build with cargo:  cargo run
// Build with rustc:  rustc ./src/main_04.rs

// Build a release version: cargo build --release
// Binary saved in: target/release/calculator

use std::env::{args, Args};

fn main() {

    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    // Build equation
    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result: f32 = operate(first_number, operator, second_number);

    println!("{:?}", output(first_number, operator, second_number, result));
}

fn operate(first_number : f32, operator: char, second_number : f32) -> f32 {
    match operator {
      '+' => first_number  + second_number,
      '-' => first_number  - second_number,
      '*' | 'x' | 'X' => first_number  * second_number,
      '/' | 'd' | 'D'  => { if second_number  != 0.0 {
                  return first_number  / second_number
               } else {
                panic!("Divide by zero")
               } },
       _ => panic!("Invalid operator")
    }
  }

fn output( first_number : f32, operator: char, second_number : f32, result: f32) -> String {
    format!("{} {} {} = {}", first_number,  operator, second_number, result)
}