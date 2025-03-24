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

// Uncomment for rand num between 1, 100 (inclusive)
fn main() {
    // let random_num: i32 = rand::thread_rng().gen_range(1..101); // Deprecated
    let random_num: i32 = rand::rng().random_range(1..101);
    println!("Random: {}", random_num);
}

