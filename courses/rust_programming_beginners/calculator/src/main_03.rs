// Build with cargo:  cargo run
// Build with rustc:  rustc ./src/main.rs

// Import the Args struct from the args function in the env module
use std::env::{args, Args};

fn main() {

    let mut args: Args = args();  // The Args struct adds variable type
                                  //  args has to be a mutable variable

    // Equation parameters
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    // Build equation
    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    println!("{:?}", operate(operator, first_number, second_number));
}

fn operate(operator: char, f_num: f32, s_num: f32) -> f32 {
    if operator == '+' {
      f_num + s_num
    } else if operator == '-' {
      f_num - s_num
    } else if operator == '*' {
      f_num * s_num                 // Need to \* on command line (not with cargo)
    } else if operator == '/' {
        if s_num != 0.0 {
        return f_num / s_num        // Need to \\ on command line with exe file
        } else {
          0.0
        }
    } else {
    0.0
    }
}

