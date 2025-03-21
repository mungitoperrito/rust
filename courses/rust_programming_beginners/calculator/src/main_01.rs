// Build with cargo:  cargo run
// Build with rustc:  rustc ./src/main_01.rs
// Run rustc output: ./main_01 one two three

// Import the Args struct from the args function in the env module
use std::env::{args, Args};

fn main() {

    let mut args: Args = args();  // The Args struct adds variable type
                                  //  args has to be a mutable variable

    // Example of print macro
    println!("ARGS: {:?}", args);

    // Example using positional parameters
    let one = args.nth(1);
    println!("nth 01: {:?}", one);
    let two = args.nth(0).unwrap();  // Panics if unwrap() has a None value
    println!("nth: O2: {:?}", two);

}

