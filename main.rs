use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;
use reqwest::blocking::Client;

fn generate_password(length: usize) -> String {
    // Generate the first third of the password using a cryptographically secure random number generator
    let mut rng = StdRng::from_entropy();
    let mut random_bytes = vec![0u8; length/3];
    rng.fill_bytes(&mut random_bytes);
    let password_part1 = hex::encode(random_bytes);

    // Generate the second third of the password using the ANU Quantum Random Generator API
    let client = Client::new();
    let url = format!("https://qrng.anu.edu.au/API/jsonI.php?length={}&type=uint8", length/3);
    let response = client.get(&url).send().unwrap();
    let data = response.json::<serde_json::Value>().unwrap();
    let random_bytes = data["data"].as_array().unwrap().iter()
        .map(|x| x.as_u64().unwrap() as u8)
        .collect::<Vec<u8>>();
    let password_part2 = base64::encode(&random_bytes);

    // Generate the final third of the password using the Random.org API
    let client = Client::new();
    let url = format!("https://www.random.org/integers/?num={}&min=0&max=255&col=1&base=10&format=plain&rnd=new", length/3);
    let response = client.get(&url).send().unwrap();
    let random_bytes = response.text().unwrap().trim().to_owned();
    let password_part3 = base64::encode(&random_bytes);

    // Concatenate the three password parts to create the final password
    let password = format!("{}{}{}", password_part1, password_part2, password_part3);
    password
}

fn main() {
    let password = generate_password(24);
    println!("{}", password);
}

