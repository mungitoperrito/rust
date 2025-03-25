#![allow(unused)]           // Suppress unused variable warnings

use std::cmp::Ordering;
use std::fs::File;
use rand::Rng;
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};


// Tuples
fn main(){
    let tuple_01: (u8, String, f64) = (47, "Yoda".to_string(), 50000.00);

    println!("Name: {}", tuple_01.1);  // Reference value

    // Type assignment inside the tuple fails
    // let (val_01: u8, val_02: String, val_03: f64) = tuple_01;
    let (val_01, val_02, val_03) = tuple_01;      // Works
    println!("val_03: {}", val_03);
}
