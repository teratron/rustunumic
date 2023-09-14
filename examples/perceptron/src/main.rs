use rustunumic::{Rustunumic, float::*};
//use rustunumic::Rustunumic;
//use rustunumic::float::FloatingPoint;

fn main() {
    // Creat instance
    let rn = Rustunumic::<f32>::new();
    //rn.calculate_neurons();
    println!("{}", rn.rate.type_name());

    //let (num, loss) = rn.train(vec![1., 2., 3.], vec![1., 2., 3.]);
    //println!("{:#?} {:#?}", num, loss);
    //println!("{:#?} {:#?}", rn, Rustunumic::SIGMOID)
}
