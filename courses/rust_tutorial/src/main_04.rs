#![allow(unused)]           // Suppress unused variable warnings

use std::cmp::Ordering;
use std::fs::File;
use rand::Rng;
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};


// Uncomment for rand num between 1, 100 (inclusive)
fn main() {
    // let random_num: i32 = rand::thread_rng().gen_range(1..101); // Deprecated
    let random_num: i32 = rand::rng().random_range(1..101);
    println!("Random: {}", random_num);
}

