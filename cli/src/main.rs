#![crate_type = "bin"]
#![crate_name = "rustunumic"]

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenv_vault::dotenv();

    let hello = env::var("HELLO").unwrap_or("".to_string());
    println!("Hello {hello}");

    Ok(())
}
