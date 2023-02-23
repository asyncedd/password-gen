// src/generator/pseudo.rs

use rand::Rng;
use std::fs;

pub fn generate_password(length: usize) -> String {
    // Generate the first third of the password using the Rust PRNG
    let mut rng = rand::thread_rng();

    // read the contents of the file
    let contents = fs::read_to_string("chars.txt").expect("Failed to read file");

    // create a Vec<char> from the contents
    let chars = contents.chars().collect::<Vec<_>>();
    let password_part1: String = (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect();

    password_part1
}

