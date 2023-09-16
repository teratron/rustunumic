mod float_struct {
    // Интерфейс для ограничения по типу floating point
    pub trait Float {}
    impl Float for f32 {}
    impl Float for f64 {}

    #[derive(Debug)]
    // Есть какая-то структура с определёнными полями
    pub struct FloatStruct<T: Float> {
        _i: i8,
        _s: String,
        pub f: T,
    }

    // Часть кода выводящего ошибку
    /*impl<T: Float> FloatStruct<T> {
        pub fn new() -> Self {
            Self {
                _i: 2,
                _s: String::from("hi"),
                f: 0.3,  // error[E0308]: mismatched types
            }
        }
    }*/

    impl FloatStruct<f32> {
        pub fn new() -> Self {
            Self {
                _i: 2,
                _s: String::from("hi"),
                f: 0.3,
            }
        }
    }

    impl FloatStruct<f64> {
        pub fn new() -> Self {
            Self {
                _i: 2,
                _s: String::from("hi"),
                f: 0.3,
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
    println!("{:#?} {:#?}", fs_32.f, fs_64.f)
}
