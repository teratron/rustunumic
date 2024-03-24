//! Synapse
//!
//!

use crate::axon::Axon;

#[repr(u8)]
pub(super) enum SynapseKind {
    Incoming = 0,
    Outgoing,
}

pub(super) trait Synapse {
    fn get_incoming_axons(&mut self) -> &Vec<Axon>;
    fn get_outgoing_axons(&mut self) -> &Vec<Axon>;
    //fn get_kind(&self, axons: SynapseKind) -> &Vec<Axon>;
    //axon.get_kind(SynapseKind.Incoming);
}

pub(super) trait SynapseOutgoing: Synapse {
    fn get_incoming_axons(&mut self) -> &Vec<Axon>;
    fn get_outgoing_axons(&mut self) -> &Vec<Axon>;
}

impl Synapse for Vec<Axon> {
    fn get_incoming_axons(&mut self) -> &Self {
        self
    }

    fn get_outgoing_axons(&mut self) -> &Self {
        self
    }
}

impl Synapse for (Vec<Axon>, Vec<Axon>) {
    fn get_incoming_axons(&mut self) -> &Vec<Axon> {
        &self.0
    }

    fn get_outgoing_axons(&mut self) -> &Vec<Axon> {
        &self.1
    }
}

/*pub(super) enum Synapse {
    Incoming(Vec<Axon>),
    Outgoing(Vec<Axon>),
}

struct Synapse {
    incoming_axons: Vec<Axon>,
    outgoing_axons: Option<Vec<Axon>>,
}*/
