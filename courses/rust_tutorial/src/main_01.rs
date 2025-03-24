#![allow(unused)]           // Suppress unused variable warnings

use std::cmp::Ordering;
use std::fs::File;
use rand::Rng;
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};


fn main() {
    println!("What's your name? ");
    let mut name: String = String::new();    // variables non-mutable by default
    io::stdin().read_line(&mut name)
        .expect("No input");

    let greeting: &str = "Pleased to meetcha";
    println!("Hello, {}! {}", name.trim_end(), greeting); // ! indicates a macro
}
