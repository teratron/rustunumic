#![crate_type = "bin"]
#![crate_name = "rustunumic"]

use std::error::Error;
use dotenvy::dotenv;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let hello = env::var("HELLO").unwrap_or("".to_string());
    println!("Hello {hello}");

    Ok(())
}
