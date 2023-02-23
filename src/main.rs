use rand::prelude::SliceRandom;
use zxcvbn::zxcvbn;
use std::io::{Read};
use std::fs::File;
use std::fs;
mod generator;

fn generate_password(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut password_parts = Vec::new();

    // read the contents of the file
    let contents = fs::read_to_string("chars.txt").expect("Failed to read file");

    // create a Vec<char> from the contents
    let chars = contents.chars().collect::<Vec<_>>();

    let password_part1 = generator::pseudo::generate_password(8);
    password_parts.push(password_part1);

    // Generate the second third of the password using the OS RNG
    let password_part2: String = (0..length / 3)
        .map(|_| {
            let random_byte: u8 = get_os_random_byte();
            chars[(random_byte % chars.len() as u8) as usize]
        })
        .collect();
    let password_part2 = base64::encode(&password_part2);
    password_parts.push(password_part2);

    let password_part3 = generator::randomorg::generate_password(10);

    password_parts.push(password_part3);

    // Concatenate the three password parts to create the final password
    password_parts.shuffle(&mut rng);
    let password = password_parts.join("");

    password
}

fn get_os_random_byte() -> u8 {
    let mut buffer = [0u8; 1];
    let mut random_file = File::open("/dev/urandom").expect("Failed to open /dev/urandom");
    random_file
        .read_exact(&mut buffer)
        .expect("Failed to read from /dev/urandom");
    buffer[0]
}

fn main() {
    let password = generate_password(24);
    let estimate = zxcvbn(&password, &[]).unwrap();
    println!("Password strength: {} (out of 4)", estimate.score());
    println!("Guesses: {}", estimate.crack_times().guesses());
    println!("Online throttling (100 per hour): {}", estimate.crack_times().online_throttling_100_per_hour());
    println!("Online no throttling (10 per second): {}", estimate.crack_times().online_no_throttling_10_per_second());
    println!("Offline slow hashing (10k per second): {}", estimate.crack_times().offline_slow_hashing_1e4_per_second());
    println!("Offline fast hashing (10B per second): {}", estimate.crack_times().offline_fast_hashing_1e10_per_second());
    println!("Feedback: {:?}", estimate.feedback());
    println!("Password: {}", password);
}

