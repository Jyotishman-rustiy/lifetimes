// src/main.rs

use strsplit::{StrSplit, until_char};

fn main() {
    let haystack = "a b c  d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    println!("letters = {:?}", letters);

    // Using the until_char helper
    let first_part = until_char("hello world", 'o');
    println!("until_char result = {:?}", first_part);

    // Another example with a char delimiter
    let csv = "apple,banana,mango";
    let fruits: Vec<_> = StrSplit::new(csv, ',').collect();
    println!("fruits = {:?}", fruits);
}
