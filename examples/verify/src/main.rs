use rustunumic::Rustunumic;

fn main() {
    // Returns a new neural network instance.
    let mut rn = Rustunumic::new();

    // Input dataset.
    let data_input = [0.27, 0.31, 0.52];

    // Target dataset.
    let data_target = [0.52, 0.31];

    // Getting the results of the trained network.
    let loss = rn.verify(&data_input, &data_target);
    print!("Verify: {loss}");
}
