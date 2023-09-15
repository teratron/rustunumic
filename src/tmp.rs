mod float_trait {
    pub trait Float {}
    impl Float for f32 {}
    impl Float for f64 {}
}

mod float_struct {
    use crate::float_trait::Float;

    #[derive(Debug)]
    // Есть какая-то структура с определёнными полями
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
    // Дать возможность выбора.
    // Опционально создать экземпляр объекта, который работает с f32
    let fs_32 = float_struct::FloatStruct::<f32>::new();
    // или f64
    let fs_64 = float_struct::FloatStruct::<f64>::new();
    println!("{:#?} {:#?}", fs_32.c, fs_64.c)
}
