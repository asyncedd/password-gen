use reqwest::blocking::Client;

fn generate_password(length: usize) -> String {
    let client = Client::new();
    let url = format!("https://www.random.org/integers/?num={}&min=0&max=255&col=1&base=10&format=plain&rnd=new", length);
    let response = client.get(&url).send().unwrap();
    let random_bytes = response.text().unwrap().trim().to_owned();
    let password = base64::encode(&random_bytes);
    password
}

fn main() {
    let password = generate_password(16);
    println!("{}", password);
}

