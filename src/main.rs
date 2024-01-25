use std::{error::Error};
use serde_json::{ Value};

fn main() -> Result<(), Box<dyn Error>> {
    let json_data = reqwest::blocking::get("https://catfact.ninja/fact")?.text()?;
    let v:Value  = serde_json::from_str(&json_data).expect("Unknown Error.Try Checking your Internet Connection");
    println!("{}", v["fact"]);
    Ok(())
}
