#![allow(unused)]           // Suppress unused variable warnings

use std::cmp::Ordering;
use std::fs::File;
use rand::Rng;
use std::{io, vec};
use std::io::{Write, BufReader, BufRead, ErrorKind};


// Strings

// // Uncomment for first example
// fn main(){
//  let mut str_01: String = String::new();

//  str_01.push('A');                             // Chars use ''
//  str_01.push_str(" some_string");              // Strings use ""
//  for word in str_01.split_whitespace(){
//     println!("{}", word);
//  }

//  let mut str_02: String = str_01.replace("A", "Another");
//  println!("{}", str_02);
// }

// // Uncomment for second example set - NOTE: ref to read vectors
// fn main() {
//     let str_01: String = String::from("x w o r d _ p u z z l e");
//     let mut vector_01: Vec<char> = str_01.chars().collect();

//     println!("{:?}", vector_01);   // Works here

//     vector_01.sort();
//     println!("{:?}", vector_01);   // Works here

//     vector_01.dedup();
//     println!("{:?}", vector_01);   // Works here

//     println!("{}", str_01);
//     for character in &vector_01 {      // Reference vector to avoid copy and
//         println!("{}", character);     //     move that cause ln 46 to fail
//     }

//     // Something happens after `in vector_01` call. "value borrowed after move"
//     //   Without &vector_01 in ln 40, compile fails when this line is here.
//     //   rust converts vectors to iterators with & (reference) which breaks code
//     println!("{:?}", vector_01);   // Fails here
// }


// // Uncomment for more &str reference
// fn main() {
//     let str_01: &str = "A direct assignment";          // Assigned to reference
//     let str_02: String = str_01.to_string();           // Heap allocated String
//     let str_03: String = "Some string".to_string();  // Convert from ref

//     // Printing works the same for &str and String
//     println!("str_01: {}", str_01);
//     println!("str_02: {}", str_02);

// }

// // Uncomment for slices
// fn main() {
//     let str_01: &str = "Some string";
//     let srt_slice: &str = &str_01[0..5];          // Characters 0-4, inclusive

//     println!("{}", srt_slice);
//     println!("Len: {}", srt_slice.len());
// }


// Uncomment for refs
fn main(){
    let str_01: String = "This is str_01".to_string();
    let str_02: String = " This is str_02".to_string();

    //  This works
    println!("str_01: {}", str_01);
    println!("str_02: {}", str_02);

    let str_03: String = str_01 + &str_02;

    // Next line fails. Ln 82 consumed str_01. Use reference instead
    // println!("str_01: {}", str_01);
    println!("str_02: {}", str_02);
    println!("str_03: {}", str_03);
}