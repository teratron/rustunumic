use rustunumic::{Rustunumic, *};

fn main() {
    // Creat instance.
    let rn = Rustunumic::new();

    let (num, loss) = rn.train(vec![1., 2., 3.], vec![1., 2., 3.]);
    println!("{:#?} {:#?}", num, loss);

    println!("{:#?} {:#?}", rn, Rustunumic::SIGMOID)
}
