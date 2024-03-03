//! # Axon
//!
//!
extern crate rand;

//use std::ops::{Range, RangeInclusive};
//use rand::Rng;
use rand::{thread_rng, Rng};

use crate::neuron::Neuron;

#[derive(Debug)]
pub struct Axon<'a, T> {
    /// Axon weight.
    pub weight: T,

    /// Incoming synapse.
    pub(crate) incoming: &'a Neuron<'a, T>,

    /// Outgoing synapse.
    pub(crate) outgoing: &'a Neuron<'a, T>,

    pub(crate) synapse: Synapse<'a, T>,
}

/*impl<'a, T: Float> Axon<'a, T> {
    pub(crate) fn new(inn: &'a Neuron<T>, out: &'a Neuron<T>) -> Self {
        //let mut rng = thread_rng();

        Self {
            weight: generate_random_weight::<T>(), //rng.gen_range(-0.5..=0.5), //T::ZERO,
            incoming: inn,
            outgoing: out,
            synapse: Synapse::new(inn, out)
        }
    }
}*/

/*const WEIGHT_RANGE: RangeInclusive<f64> = -0.5..=0.5;

fn generate_random_weight<T: Float>() -> T {
    let mut rng = thread_rng();

    //let v: T = 0.5 as <T as Float>::FloatType;

    rng.gen_range(WEIGHT_RANGE)
}*/

impl<'a> Axon<'a, f64> {
    pub(crate) fn new(inn: &'a Neuron<f64>, out: &'a Neuron<f64>) -> Self {
        let mut rng = thread_rng();

        Self {
            weight: rng.gen_range(-0.5..=0.5),
            incoming: inn,
            outgoing: out,
            synapse: Synapse::new(inn, out),
        }
    }
}

impl<'a> Axon<'a, f32> {
    pub(crate) fn new(inn: &'a Neuron<f32>, out: &'a Neuron<f32>) -> Self {
        let mut rng = thread_rng();

        Self {
            weight: rng.gen_range(-0.5..=0.5),
            incoming: inn,
            outgoing: out,
            synapse: Synapse::new(inn, out),
        }
    }
}

#[derive(Debug)]
struct Synapse<'a, T> {
    /// Incoming synapse.
    incoming: &'a Neuron<'a, T>,

    /// Outgoing synapse.
    outgoing: &'a Neuron<'a, T>,
}

impl<'a, T> Synapse<'a, T> {
    fn new(p0: &'a Neuron<T>, p1: &'a Neuron<T>) -> Self {
        Self {
            incoming: p0, //Neuron::new(),
            outgoing: p1, //Neuron::new()
        }
    }
}

/*enum Synapse<'a, T> {
    Incoming(&'a Neuron<'a, T>),
    Outgoing(&'a Neuron<'a, T>),
}*/

/*impl<T: Float> Default for T {
    fn default() -> Self {
        T::ZERO
    }
}*/

/*impl<'a> Axon<'a, f64> {
    pub(crate) fn new(inn: &'a Neuron<f64>, out: &'a Neuron<f64>) -> Self {
        Self {
            weight: 0.,
            incoming: inn,
            outgoing: out
        }
    }

    pub(crate) fn back(&mut self) -> f64 {
        self.incoming.value * self.weight
    }
}*/
