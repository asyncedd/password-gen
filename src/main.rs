use rand::Rng;
use reqwest::blocking::Client;
use serde_json::Value;

fn generate_password(length: usize) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_+{}|[]\\;:'\",./<>?".chars().collect::<Vec<char>>();
    let mut rng = rand::thread_rng();
    let password_part1: String = (0..length).map(|_| chars[rng.gen_range(0..chars.len())]).collect();

    // Generate the second third of the password using the ANU Quantum Random Generator API
    let client = Client::new();
    let url = format!("https://qrng.anu.edu.au/API/jsonI.php?length={}&type=uint8", length/3);
    let response = client.get(&url).send().unwrap();
    let json: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    let random_bytes = json["data"].as_array().unwrap().iter()
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

