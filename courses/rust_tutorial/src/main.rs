#![allow(unused)]           // Suppress unused variable warnings

use std::cmp::Ordering;
use std::fs::File;
use rand::Rng;
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};


// // Uncomment to see type sizes
// fn main() {
//     println!("Max u32: {}", u32::MAX);
//     println!("Max u64: {}", u64::MAX);
//     println!("Max usize: {}", usize::MAX);
//     println!("Max f32: {}", f32::MAX);
//     println!("Max f64: {}", f64::MAX);
// }

// Uncomment to see floats
fn main() {
    let _ignore_this: f32 = 123.45;   // compiler ignores: underscore var name

    let num_01: f32 = 1.111111111111111;
    println!("f32: {}", num_01 + 0.1111111111111111 );
    let num_02: f64 = 1.111111111111111;
    println!("f64: {}", num_02 + 0.1111111111111111 );
}

