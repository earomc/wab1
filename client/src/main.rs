use std::{
    error::Error,
    io::{stdin, Read}, time::Duration,
};

use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Creating client ...");
    let client = reqwest::blocking::Client::builder().timeout(Duration::from_secs(10)).build()?;
    loop {
        let mut input = String::new();
        if let Ok(_) = stdin().read_line(&mut input) {
            println!("Read string {}", input);
            let url = format!("https://api.mcsrvstat.us/3/{}", input);
            println!("{}", url);
            let request = client
                .get(url)
                .build()?;
            let response = client.execute(request);
            let mut response_string = String::new();
            response.unwrap().read_to_string(&mut response_string)?;
            println!("{}",response_string);
            let v: Value = serde_json::from_str(&response_string)?;
            println!(
                "Pinged {} \nPlayers online: {}",
                v["hostname"], v["players"]["online"]
            );
        } else {
            println!("Failed to read string");
        }
    }
}
