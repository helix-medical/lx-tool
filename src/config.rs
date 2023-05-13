use serde::{Deserialize, Serialize};
use serde_json as json;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Cabinet {
    name: String,
    address: String,
    city: String,
    phone: String,
    email: String,
    website: String,
    siret: String,
}

#[allow(non_snake_case)]
struct Config {
    defaultAmount: u32,
    defaultPaymentMethod: String,
    durationAppointment: u32,
    cabinet: Cabinet,
}

pub fn create_config() -> std::io::Result<()> {
    let mut file = File::create("config.json")?;
    let cabinet = create_cabinet();
    let buffer = json::to_string_pretty(&cabinet).unwrap();
    file.write_all(buffer.as_bytes())?;
    Ok(())
}

fn create_cabinet() -> Cabinet {
    let stdin: std::io::Stdin = std::io::stdin();

    println!(" -- Cabinet Information --");
    println!(" -- Name: ");
    let name: String = stdin.lock().lines().next().unwrap().unwrap();
    println!(" -- Address: ");
    let address: String = stdin.lock().lines().next().unwrap().unwrap();
    println!(" -- City: ");
    let city: String = stdin.lock().lines().next().unwrap().unwrap();
    println!(" -- Phone: ");
    let phone: String = stdin.lock().lines().next().unwrap().unwrap();
    println!(" -- Email: ");
    let email: String = stdin.lock().lines().next().unwrap().unwrap();
    println!(" -- Website: ");
    let website: String = stdin.lock().lines().next().unwrap().unwrap();
    println!(" -- Siret: ");
    let siret: String = stdin.lock().lines().next().unwrap().unwrap();

    Cabinet {
        name,
        address,
        city,
        phone,
        email,
        website,
        siret,
    }
}
