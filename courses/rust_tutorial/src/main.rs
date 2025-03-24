#![allow(unused)]           // Suppress unused variable warnings

use std::cmp::Ordering;
use std::fs::File;
use rand::Rng;
use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};


// // Uncomment for if .. else if .. else
// fn main() {
//     // Need to learn typing for lists
//     let ages = [0, 8, 18, 19, 21, 35, 50, 65, 80 ];

//     for age in ages  {
//         if (age >= 1) && (age <= 18) {
//             println!("{}: Important BDay", age);
//         } else if (age == 21) || (age == 50) {
//             println!("{}: Important BDay", age);
//         } else if (age >= 65) {
//             println!("{}: Important BDay", age);
//         } else {
//             println!("{}: Not important", age);
//         }
//     }
// }

// Uncomment for ternary operator
fn main(){
    let mut age: u32 = 47;
    let mut can_vote: bool = if age >= 18 {
        true             // Return statement: not 'return', no ;
    } else {
        false            // Return statement: not 'return', no ;
    };

    println!("{}", can_vote);   // Variable needs a format string
}
