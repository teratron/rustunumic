//! # Neuron
//!
//!

use crate::activation::Activation;
use crate::axon::Axon;
use crate::float::Float;

type AxonsType<'a, T> = Box<Vec<&'a Axon<'a, T>>>;

struct Cell<T> {
    /// Neuron value.
    pub value: T,

    /// Neuron error.
    pub miss: T,

    /// Function activation.
    pub activation: Option<Activation>,

    /// Is there a bias neuron.
    has_bias: bool,
}

impl<T: Float> Cell<T> {
    fn new() -> Self {
        Self {
            value: T::ZERO,
            miss: T::ZERO,
            activation: None,
            has_bias: false,
        }
    }
}

struct Input<T>(T);

struct Hidden<'a, T> {
    cell: Cell<T>,
    incoming: AxonsType<'a, T>,
    outgoing: AxonsType<'a, T>,
}

struct Output<'a, T> {
    cell: Cell<T>,
    target: T,
    incoming: AxonsType<'a, T>,
}

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

/*pub(crate) enum CellKind<'a, T> {
    Input(T),
    BackfedInput,
    NoisyInput,

    Hidden(Cell<T>, Incoming<'a, T>, Outgoing<'a, T>),
    ProbabilisticHidden,
    SpikingHidden,
    Capsule,
    Bias(bool),

    Output(Cell<T>, Target<T>, Incoming<'a, T>),
    MatchInputOutput,

    Recurrent,
    Memory,
    GatedMemory,

    Kernel,
    Convolution, // or Pool
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
