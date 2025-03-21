// Build with cargo:  cargo run
// Build with rustc:  rustc ./src/main_02.rs
// Run rustc output: ./main_02 1 + 2


// Import the Args struct from the args function in the env module
use std::env::{args, Args};

fn main() {

    let mut args: Args = args();  // The Args struct adds variable type
                                  //  args has to be a mutable variable

    // Equation parameters
    // First call to nth starts the sequence. After that, 0 gets the next element
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    // Uncomment to debug
    println!("{:?} {} {}", first, operator, second);


    // Build equation
    let first_number = first.parse::<f32>().unwrap();     // Alternate type specification method
    let second_number: f32 = second.parse().unwrap();     // Equivalent specification method

    // Uncomment to debug
    println!("{:?} {} {}", first_number, operator, second_number);
}
