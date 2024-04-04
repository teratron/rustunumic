//! Synapse
//!
//!

use super::axon::Axon;

pub(super) trait SynapseIncoming<T> {
    fn get_incoming_axons(&mut self) -> &Vec<Axon<T>>;
}

pub(super) trait Synapse<T>: SynapseIncoming<T> {
    fn get_outgoing_axons(&mut self) -> &Vec<Axon<T>>;
}

// For type Vec<Axon<T>>
impl<T> SynapseIncoming<T> for Vec<Axon<T>> {
    fn get_incoming_axons(&mut self) -> &Self {
        self
    }
}

// For type (Vec<Axon<T>>, Vec<Axon<T>>)
impl<T> SynapseIncoming<T> for (Vec<Axon<T>>, Vec<Axon<T>>) {
    fn get_incoming_axons(&mut self) -> &Vec<Axon<T>> {
        &self.0
    }
}

impl<T> Synapse<T> for (Vec<Axon<T>>, Vec<Axon<T>>) {
    fn get_outgoing_axons(&mut self) -> &Vec<Axon<T>> {
        &self.1
    }
}
