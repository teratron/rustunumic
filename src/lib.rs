#![crate_name = "rustunumic"]
/*#![deny(
    rust_2021_compatibility,
    future_incompatible,
    nonstandard_style,
    clippy::all,
    clippy::doc_markdown,
    missing_docs,
    missing_copy_implementations,
    missing_debug_implementations
)] //unused,
*/

//! # Simple neural network library for Rust.
//!
//! This is the neural network library.
//!
//! For examples, see [examples](https://github.com/zigenzoog/rustunumic/examples).

// Reexported crates.
// pub use event_loop::{self, *};
// pub use input::{self, *};
// pub use window::{self, *};

pub mod activation;
mod axon;
mod neuron;

use activation::Activation;

#[derive(Debug)]
pub struct Rustunumic {
    //activation: Activation,
}

impl Rustunumic {
    /// Creat new
    pub fn new() -> Self {
        Rustunumic {
            //activation: Activation::SIGMOID,
        }
    }

    // pub fn sigmoid(&self) -> Activation {
    //     Activation::SIGMOID
    // }

    pub const SIGMOID: Activation = Activation::SIGMOID;
}

trait Traiter {}

impl Traiter for Rustunumic {}

impl Traiter for Activation {}

/*pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4, "failed test");
    }
}

fn main() {
    let a = 6;
    let b = 8;
    println!("{}", add(a, b))
}*/
