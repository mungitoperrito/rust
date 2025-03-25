#![allow(unused)]           // Suppress unused variable warnings

use std::cmp::Ordering;
use std::fs::File;
use rand::Rng;
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};


// Uncomment for arrays
fn main(){
    let array_01: [i32; 4] = [1,2,3,4];

    // // Uncomment for an array
    // println!("1st: {}", array_01[0]);
    // println!("Length: {}", array_01.len());

    // // Uncomment for a loop
    // let array_02: [i32; 9] = [1,2,3,4,5,6,7,8,9];
    // let mut idx: usize = 0;
    // loop {
    //     if array_02[idx] % 2 == 0 {
    //         println!("{}", array_02[idx]);
    //         idx += 1;
    //     } else {
    //         idx += 1;
    //         continue;
    //     }
    //     if idx == 9 {
    //         break;
    //     }

    // }

    // // Uncomment for while loop
    // let array_02: [i32; 9] = [1,2,3,4,5,6,7,8,9];
    // let mut idx: usize = 0;
    // while idx < array_02.len(){
    //     println!("Array: {}", array_02[idx]);
    //     idx += 1;
    // }


    // Uncomment for for loop
    for val in array_01.iter() {      // autocomplete suggestion val: &i32 fails
        println!("Val: {}", val);
    }
}
