//! Synapse
//!
//!

use crate::axon::Axon;

#[repr(u8)]
pub(super) enum SynapseKind {
    Incoming = 0,
    Outgoing,
}

pub(super) trait SynapseIncoming {
    fn get_incoming_axons(&mut self) -> &Vec<Axon>;
}

pub(super) trait Synapse: SynapseIncoming {
    fn get_outgoing_axons(&mut self) -> &Vec<Axon>;
}

// For Vec<Axon>
impl SynapseIncoming for Vec<Axon> {
    fn get_incoming_axons(&mut self) -> &Self {
        self
    }
}

// For (Vec<Axon>, Vec<Axon>)
impl SynapseIncoming for (Vec<Axon>, Vec<Axon>) {
    fn get_incoming_axons(&mut self) -> &Vec<Axon> {
        &self.0
    }
}

impl Synapse for (Vec<Axon>, Vec<Axon>) {
    fn get_outgoing_axons(&mut self) -> &Vec<Axon> {
        &self.1
    }
}
