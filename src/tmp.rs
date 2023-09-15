mod float_trait {
    pub trait Float {}
    impl Float for f32 {}
    impl Float for f64 {}
}

mod float_struct {
    use crate::float_trait::Float;

    #[derive(Debug)]
    pub struct FloatStruct<T: Float> {
        _a: i8,
        _b: String,
        pub c: T,
    }

    /*impl<T: Float> FloatStruct<T> {
        pub fn new() -> Self {
            Self {
                _a: 2,
                _b: String::from("hi"),
                c: 0.3,  // error[E0308]: mismatched types
            }
        }
    }*/

    impl FloatStruct<f32> {
        pub fn new() -> Self {
            Self {
                _a: 2,
                _b: String::from("hi"),
                c: 0.3,
            }
        }
    }

    impl FloatStruct<f64> {
        #![allow(dead_code)]
        pub fn new() -> Self {
            Self {
                _a: 2,
                _b: String::from("hi"),
                c: 0.3,
            }
        }
    }
}

fn main() {
    let fs = float_struct::FloatStruct::<f32>::new();
    println!("{:#?}", fs.c)
}
