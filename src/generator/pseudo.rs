// src/generator/pseudo.rs

use rand::Rng;

pub fn generate_password(length: usize, chars: &[char]) -> String {
    // Generate the first third of the password using the Rust PRNG
    let mut rng = rand::thread_rng();

        let password_part1: String = (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect();

    password_part1
}

