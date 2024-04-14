//! # Synapse
//!
//!

use super::axon::Axon;

type AxonBundle<T> = Vec<Axon<T>>;

// Массив входящих аксонов для нейрона.
pub(super) trait SynapseIncoming<T> {
    fn get_incoming_axons(&mut self) -> &AxonBundle<T>;
}

// Массив исходящих аксонов для нейрона.
pub(super) trait SynapseOutgoing<T> {
    fn get_outgoing_axons(&mut self) -> &AxonBundle<T>;
}

pub(super) trait Synapse<T>: SynapseIncoming<T> + SynapseOutgoing<T> {}

// For type Vec<Axon<T>>
impl<T> SynapseIncoming<T> for AxonBundle<T> {
    fn get_incoming_axons(&mut self) -> &Self {
        self
    }
}

// For type (Vec<Axon<T>>, Vec<Axon<T>>)
impl<T> SynapseIncoming<T> for (AxonBundle<T>, AxonBundle<T>) {
    fn get_incoming_axons(&mut self) -> &AxonBundle<T> {
        &self.0
    }
}

impl<T> SynapseOutgoing<T> for (AxonBundle<T>, AxonBundle<T>) {
    fn get_outgoing_axons(&mut self) -> &AxonBundle<T> {
        &self.1
    }
}
