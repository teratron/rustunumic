//! # Neuron
//!
//!

use crate::activation::{get_activation, Activation};
use crate::axon::Axon;

#[derive(Debug)]
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

pub(crate) struct CellInput<'a, T> {
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

    bias: bool,

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

    bias: bool,

    /// Function activation.
    pub activation: Option<Activation>,
}

trait Neurons<T> {
    fn calculate_value(&mut self);
    fn calculate_miss(&mut self);
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum CellKind2<'a, T> {
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
}

/*impl<'a> CellKind2<'a, f64> {
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

type AxonsType<'a, T> = Box<Vec<&'a Axon<'a, T>>>;

pub(crate) struct Cell<T> {
    value: T,
}

pub(crate) struct Target<T>(T);

pub(crate) enum CellKind<'a, T> {
    // T - value of neuron, AxonsType<'a, T> - outgoing axons
    Input(Cell<T>, AxonsType<'a, T>),
    BackfedInput,
    NoisyInput,

    Hidden,
    ProbabilisticHidden,
    SpikingHidden,
    Capsule,
    Bias(bool),

    // T - target
    //Output(T, Neuron<'a, T>),
    Output(Cell<T>, Target<T>, AxonsType<'a, T>),
    MatchInputOutput,

    Recurrent,
    Memory,
    GatedMemory,

    Kernel,
    Convolution, // or Pool
}
