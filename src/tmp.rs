mod float_struct {
    use std::marker::PhantomData;

    // Интерфейс для ограничения по типу floating point
    pub trait Float {
        type FloatType;
    }

    impl Float for f32 {
        type FloatType = f32;
    }

    impl Float for f64 {
        type FloatType = f64;
    }

    #[derive(Debug)]
    // Есть какая-то структура с определёнными полями
    pub struct FloatStruct<T: Float> {
        phantom: PhantomData<T>,
        _i: i8,
        _s: String,
        pub f: T,
    }

    // Часть кода выводящего ошибку
    impl<T: Float> FloatStruct<T> {
        pub fn new() -> Self {
            Self {
                phantom: PhantomData,
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
    println!("{:#?} {:#?}", fs_32.f, fs_64.f);

    let a = vec![1, 2, 3];
    let b = &a[1..3];
}

/*impl PartialEq for Loss {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}*/

/*
trait Synapse {}

impl Synapse for Vec<Axon> {}

impl Synapse for (Vec<Axon>, Vec<Axon>) {}

pub(super) enum Synapse {
    Incoming(Vec<Axon>),
    Outgoing(Vec<Axon>),
}

struct Synapse {
    incoming_axons: Vec<Axon>,
    outgoing_axons: Option<Vec<Axon>>,
}

pub(crate) enum CellKind<'a, T> {
    Input(T),
    BackfedInput,
    NoisyInput,

    //Hidden(Cell<T>, Incoming<'a, T>, Outgoing<'a, T>),
    Hidden(Hidden<'a, T>),
    ProbabilisticHidden,
    SpikingHidden,
    Capsule,
    Bias(bool),

    //Output(Cell<T>, Target<T>, Incoming<'a, T>),
    Output(Output<'a, T>),
    MatchInputOutput,

    Recurrent,
    Memory,
    GatedMemory,

    Kernel,
    Convolution, // or Pool
}*/
