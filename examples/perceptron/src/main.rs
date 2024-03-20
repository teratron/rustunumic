#![allow(unused)]

use rustunumic::{activation::Activation, loss::Loss, Rustunumic};

struct Perceptron<T> {
    bias: bool,
    rate: T,
    hidden_layers: Vec<usize>,
    activation: Activation,
    loss: Loss,
}

fn main() {
    let perceptron = Perceptron::<f64> {
        bias: true,
        rate: 0.3,
        hidden_layers: vec![3, 2],
        activation: Activation::TanH,
        loss: Loss::MSE,
    };

    // Creat instance
    //let mut rn = Rustunumic::<f32>::new();
    let mut rn = Rustunumic::new();
    rn.calculate_neurons();
    //let (num, loss) = rn.train(vec![1., 2., 3.], vec![1., 2., 3.]);
    //println!("{:#?} {:#?}", num, loss);
    //println!("{:#?} {:#?}", rn, Rustunumic::SIGMOID)
}
