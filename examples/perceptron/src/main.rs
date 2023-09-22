use rustunumic::{float::*, Rustunumic};
//use rustunumic::Rustunumic;
//use rustunumic::float::FloatingPoint;

fn main() {
    // Creat instance
    let mut rn = Rustunumic::<f32>::new();
    rn.rate = 0.5;
    println!("{}", rn.rate.type_name());

    let mut rs = Rustunumic::<f64>::new();
    println!("{}", rs.rate.type_name());

    //rn.calculate_neurons();
    //let (num, loss) = rn.train(vec![1., 2., 3.], vec![1., 2., 3.]);
    //println!("{:#?} {:#?}", num, loss);
    //println!("{:#?} {:#?}", rn, Rustunumic::SIGMOID)
}
