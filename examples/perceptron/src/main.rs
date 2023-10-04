use rustunumic::{activation::Activation, float::Float, loss::Loss, Rustunumic};

struct Perceptron<T> {
    bias: bool,
    rate: T,
    hidden_layers: Vec<usize>,
    activation: Activation,
    loss: Loss,
}

/*impl Perceptron<f32> {
    fn new() -> Self {
        Self {
            bias: true,
            rate: 0.3,
            hidden_layers: Vec::new(),
            activation: Activation::TanH,
            loss: Loss::MSE,
        }
    }

}*/

fn main() {
    let perceptron = Perceptron::<f32> {
        bias: true,
        rate: 0.3,
        hidden_layers: vec![3, 2],
        activation: Activation::TanH,
        loss: Loss::MSE,
    };

    // Creat instance
    let mut rn = Rustunumic::<f32>::new();
    rn.rate = 0.5;
    println!("{:?}", rn.rate.type_name());

    let mut rs = Rustunumic::<f64>::new();
    println!("{:?}", rs.rate.type_name());

    //rn.calculate_neurons();
    //let (num, loss) = rn.train(vec![1., 2., 3.], vec![1., 2., 3.]);
    //println!("{:#?} {:#?}", num, loss);
    //println!("{:#?} {:#?}", rn, Rustunumic::SIGMOID)
}
