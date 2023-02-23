use reqwest::blocking::Client;
use rand::prelude::*;

pub fn generate_password(length: usize, chars: &Vec<char>) -> String {

    let mut rng = rand::thread_rng();
    // Generate the final third of the password using the Random.org API
    let client = Client::new();

    let url = format!(
        "https://www.random.org/integers/?num={}&min=0&max=255&col=1&base=10&format=plain&rnd=new",
        length
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


    password_part3

}

