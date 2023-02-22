use rand::Rng;
use rand::prelude::SliceRandom;
use rand::prelude::*;
use reqwest::blocking::Client;
use zxcvbn::zxcvbn;
use std::io::{Read};
use std::fs::File;
use RustPasswordGen::generator::psuedo;

fn generate_password(length: usize) -> String {
    let mut password_parts = Vec::new();

    let password_part1 = psuedo(8);
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

    // Generate the final third of the password using the Random.org API
    let client = Client::new();
    let url = format!(
        "https://www.random.org/integers/?num={}&min=0&max=255&col=1&base=10&format=plain&rnd=new",
        length / 3
    );
    let response = client.get(&url).send().unwrap();
    let random_bytes = response.text().unwrap().trim().to_owned();

    let random_integers: Vec<usize> = random_bytes
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let random_index = *random_integers.choose(&mut rng).unwrap();

    let mut password_part3: Vec<char> = chars.clone();
    password_part3.shuffle(&mut rng);
    let password_part3 = password_part3.iter().cycle().skip(random_index).take(length / 3).collect::<String>();
    // let password_part3 = base64::encode(&password_part3);
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

