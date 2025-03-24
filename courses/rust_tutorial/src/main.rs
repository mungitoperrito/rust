#![allow(unused)]           // Suppress unused variable warnings

use std::cmp::Ordering;
use std::fs::File;
use rand::Rng;
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};


fn main() {
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);

}
