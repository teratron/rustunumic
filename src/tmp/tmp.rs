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
        pub f: <f64 as Float>::FloatType,
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
    println!("{:#?} {:#?}", fs_32.f, fs_64.f)
}

/*fn calculate_miss(&mut self) {
    let mut n: usize = self.outgoing_axons_last_index;
    while n >= 0 {
        self.outgoing_axons[n].calculate_miss(self);
        n -= 1;
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

struct CoreCell {
    /// Neuron value.
    value: f32,

    /// Neuron error.
    miss: f32,

    /// Function activation mode.
    activation_mode: Option<Activation>,

    /// All incoming axons.
    incoming_axons: Vec<Axon>,
    //&'a
    _rate: f32,

    // incoming_axons or (incoming_axons, outgoing_axons)
    // Vec<Axon> or (Vec<Axon>, Vec<Axon>)
    // (Vec<Axon>, Option<Vec<Axon>>)
    synapses: dyn Synapse,

    // HiddenCell, OutputCell
    cell: dyn KindTrait,
}

impl CoreCell {
    fn activation(&mut self) {}
    fn derivative(&mut self) {}

    // Forward propagation.
    fn calculate_value(&mut self) {
        self.value = 0.;
        for axon in self.incoming_axons {
            self.value += axon.calculate_value();
        }
    }

    // Backward propagation.
    fn update_weight(&mut self) {
        let gradient = self._rate
            * self.miss
            * get_derivative(&mut (self.value as f64), &self.activation_mode.unwrap());

        for axon in &mut self.incoming_axons {
            axon.update_weight(&gradient);
        }
    }
}

impl CoreTrait for CoreCell {
    fn get_value(&self) -> &f32 {
        &self.value
    }
}

impl CellTrait for CoreCell {
    fn get_miss(&self) -> &f32 {
        &self.miss
    }

    fn set_miss(&mut self, value: f32) {
        self.miss = value;
    }
}*/
