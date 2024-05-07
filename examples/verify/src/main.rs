#![allow(unused)]

use std::time;

use rustunumic::Rustunumic;

fn main() {
    // Start time.
    let now = time::Instant::now();

    // Returns a new neural network instance.
    let mut rn = Rustunumic::<f64>::new();
    println!("{:#?}", rn);

    // Input dataset.
    //let data_input = [0.27, 0.31, 0.52];

    // Target dataset.
    //let data_target = [0.52, 0.31];

    // Getting the results of the trained network.
    //let loss = rn.verify(&data_input, &data_target);
    //print!("Verify: {loss}");

    //let a = 0.0..27.0;

    // Elapsed time.
    println!("Elapsed time: {} microseconds.", now.elapsed().as_micros());
}
