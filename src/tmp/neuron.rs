//! # Neuron
//!
//!

use crate::activation::Activation;
use crate::axon::Axon;
use crate::float::Float;

//type AxonsType<'a, T> = Box<Vec<Axon<'a, T>>>;
type AxonsType<T> = Box<Vec<Axon<T>>>;

pub(crate) trait Nucleus<T> {
    fn get_value(&self) -> &T;
}

pub trait Neuron<'a, T>: Nucleus<T> {
    fn get_miss(&self) -> &T;
}

pub(crate) trait Synapse<T>: Nucleus<T> {}

struct Core<T> {
    /// Neuron value.
    value: T,

    /// Neuron error.
    miss: T,

    /// Function activation.
    activation: Option<Activation>,

    /// Is there a bias neuron.
    has_bias: bool,
}

impl<T: Float> Core<T> {
    fn new() -> Self {
        Self {
            value: T::ZERO,
            miss: T::ZERO,
            activation: None,
            has_bias: false,
        }
    }

    fn add(&mut self, activation: Option<Activation>, bias: bool) -> &Self {
        self.activation = activation;
        self.has_bias = bias;
        self
    }

    fn get_value(&self) -> &T {
        &self.value
    }

    fn get_miss(&self) -> &T {
        &self.value
    }

    /*pub(crate) fn calculate_neuron(&mut self) {
        for neuron in self.incoming.iter_mut() {
            neuron.calculate_value();
        }
    }*/
}

// Input neuron.
struct Input<T>(T);

impl<T: Float> Nucleus<T> for Input<T> {
    fn get_value(&self) -> &T {
        &self.0
    }
}

// Hidden neuron.
struct Hidden<T> {
    cell: Core<T>,
    incoming: AxonsType<'_, T>,
    outgoing: AxonsType<'_, T>,
}

impl<T: Float> Hidden<T> {
    fn new() -> Self {
        Self {
            cell: Core::new(),
            incoming: Box::new(Vec::new()),
            outgoing: Box::new(Vec::new()),
        }
    }

    pub(crate) fn calculate_neuron(&mut self) {
        for axon in self.incoming.iter() {
            let _s = axon.calculate_forward_value();
        }
    }
}

impl<T: Float> Nucleus<T> for Hidden<T> {
    fn get_value(&self) -> &T {
        &self.cell.value
    }
}

impl<T: Float> Neuron<'_, T> for Hidden<T> {
    fn get_miss(&self) -> &T {
        &self.cell.miss
    }
}

// Output neuron.
struct Output<T> {
    cell: Core<T>,
    target: T,
    incoming: AxonsType<T>,
}

impl<T: Float> Nucleus<T> for Output<T> {
    fn get_value(&self) -> &T {
        &self.cell.value
    }
}

impl<T: Float> Neuron<'_, T> for Output<T> {
    fn get_miss(&self) -> &T {
        &self.cell.miss
    }
}

/*pub(crate) enum CellKind<'a, T> {
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

/*struct Incoming<'a, T>(AxonsType<'a, T>);

struct Outgoing<'a, T>(AxonsType<'a, T>);

struct Target<T>(T);*/

/*trait Neuron<T> {
    fn new() -> Self;
    fn calculate_value(&mut self);
    fn calculate_error(&mut self, target: &Target<T>);
    fn calculate_gradient(&mut self);
    fn calculate_delta(&mut self, target: &Target<T>);
    fn calculate_weight(&mut self, target: &Target<T>);
    fn calculate_bias(&mut self, target: &Target<T>);
    fn calculate_activation(&mut self);
    fn calculate_miss(&mut self);
    fn calculate_error_gradient(&mut self, target: &Target<T>);
    fn calculate_error_delta(&mut self, target: &Target<T>);
    fn calculate_error_weight(&mut self, target: &Target<T>);
    fn calculate_error_bias(&mut self, target: &Target<T>);
    fn calculate_error_activation(&mut self, target: &Target<T>);
    fn calculate_error_miss(&mut self, target: &Target<T>);
    fn calculate_error_error(&mut self, target: &Target<T>);
}*/

/*#[derive(Debug)]
pub(crate) struct Neuron<'a, T> {
    /// Neuron value.
    pub value: T,

    /// Neuron error.
    pub miss: T,

    /// All incoming axons.
    pub incoming: Box<Vec<Axon<'a, T>>>,

    /// All outgoing axons.
    pub outgoing: Box<Vec<&'a Axon<'a, T>>>,

    /// Function activation.
    pub activation: Option<Activation>,
}

impl<'a, T: Float> Neuron<'a, T> {
    pub(crate) fn new() -> Self {
        Self {
            value: T::ZERO,
            miss: T::ZERO,
            incoming: Box::new(Vec::new()), //.push(Axon<'a, f64>::new()),
            outgoing: Box::new(Vec::new()),
            activation: None,
        }
    }
}

impl<'a> Neuron<'a, f64> {
    pub(crate) fn new() -> Self {
        Self {
            value: 0.,
            miss: 0.,
            incoming: Box::new(Vec::new()), //.push(Axon<'a, f64>::new()),
            outgoing: Box::new(Vec::new()),
            activation: None,
        }
    }

    pub(crate) fn calculate_value(&mut self) {
        self.value = 0.;
        for axon in self.incoming.iter() {
            self.value += axon.incoming.value * axon.weight;
        }

        if let Some(mode) = &self.activation {
            //self.value = activation(&self.value, mode);
            get_activation(&mut self.value, mode);
        }
    }

    pub(crate) fn calculate_loss(&mut self) {}

    pub(crate) fn calculate_miss(&mut self) {}

    pub(crate) fn update_weights(&mut self) {}
}


trait Neurons<T> {
    fn calculate_value(&mut self);
    fn calculate_miss(&mut self);
}*/

/*pub(crate) struct CellInput<'a, T> {
    /// Neuron value.
    value: T,

    /// All outgoing axons.
    outgoing: Box<Vec<&'a Axon<'a, T>>>,
}

pub(crate) struct CellOutput<'a, T> {
    /// Neuron value
    value: T,

    /// Neuron error
    miss: T,

    /// Target neuron
    target: T,

    /// All incoming axons
    incoming: Box<Vec<Axon<'a, T>>>,

    has_bias: bool,

    /// Function activation
    activation: Activation,
}

pub(crate) struct CellHidden<'a, T> {
    /// Neuron value.
    pub value: T,

    /// Neuron error.
    pub miss: T,

    /// All incoming axons.
    pub incoming: Box<Vec<Axon<'a, T>>>,

    /// All outgoing axons.
    pub outgoing: Box<Vec<&'a Axon<'a, T>>>,

    has_bias: bool,

    /// Function activation.
    pub activation: Option<Activation>,
}*/

/*#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum CellKind<'a, T> {
    Input {
        /// Neuron value.
        value: T,

        /// All outgoing axons.
        outgoing: Box<Vec<&'a Axon<'a, T>>>,
    },

    Hidden {
        /// Neuron value.
        value: T,

        /// Neuron error.
        miss: T,

        /// All incoming axons.
        incoming: Box<Vec<Axon<'a, T>>>,

        /// All outgoing axons.
        outgoing: Box<Vec<&'a Axon<'a, T>>>,

        /// Function activation.
        activation: Option<Activation>,
    },

    Bias(bool),

    Output {
        /// Neuron value
        value: T,

        /// Neuron error
        miss: T,

        /// Target neuron
        target: T,

        /// All incoming axons
        incoming: Box<Vec<Axon<'a, T>>>,

        /// Function activation
        activation: Activation,
    },
}*/

/*impl<'a> CellKind<'a, f64> {
    pub(crate) fn calculate_value(&mut self) {
        match self {
            CellKind2::Input {value, outgoing} => todo!(),
            CellKind2::Hidden { value, miss, incoming, outgoing, activation } => todo!(),
            CellKind2::Bias(_) => todo!(),
            CellKind2::Output { target, value, miss, incoming, activation } => todo!(),
        }
        //self::Hidden.value = 0.;
        for axon in self.incoming.iter() {
            self.value += axon.incoming.value * axon.weight;
        }

        if let Some(mode) = &self.activation {
            //self.value = activation(&self.value, mode);
            get_activation(&mut self.value, mode);
        }
    }
}*/
