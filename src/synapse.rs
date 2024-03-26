//! Synapse
//!
//!

use crate::float::Float;

use super::axon::Axon;

pub(super) trait SynapseIncoming<T: Float> {
    fn get_incoming_axons(&mut self) -> &Vec<Axon<T>>;
}

pub(super) trait Synapse<T: Float>: SynapseIncoming<T> {
    fn get_outgoing_axons(&mut self) -> &Vec<Axon<T>>;
}

// For type Vec<Axon<T>>
impl<T: Float> SynapseIncoming<T> for Vec<Axon<T>> {
    fn get_incoming_axons(&mut self) -> &Self {
        self
    }
}

// For type (Vec<Axon<T>>, Vec<Axon<T>>)
impl<T: Float> SynapseIncoming<T> for (Vec<Axon<T>>, Vec<Axon<T>>) {
    fn get_incoming_axons(&mut self) -> &Vec<Axon<T>> {
        &self.0
    }
}

impl<T: Float> Synapse<T> for (Vec<Axon<T>>, Vec<Axon<T>>) {
    fn get_outgoing_axons(&mut self) -> &Vec<Axon<T>> {
        &self.1
    }
}
