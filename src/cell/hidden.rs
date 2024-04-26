//! # Hidden Cell
//!
//!

use std::fmt::{Debug /*, Formatter, Result*/};

use crate::axon::AxonBundle;
use crate::{Activation, Float};

use super::{CoreCell, Neuron, Nucleus};

#[derive(Debug)]
pub(crate) struct HiddenCell<T> {
    /// Core cell.
    core: CoreCell<T>,

    /// Outgoing axons.
    outgoing_axons: AxonBundle<T>,
}

impl<T: Float> HiddenCell<T> {
    pub(crate) fn new(activation_mode: Activation) -> Self {
        Self {
            core: CoreCell::new(activation_mode),
            outgoing_axons: Vec::new(),
        }
    }

    /*pub(crate) fn calculate_miss(&mut self) {
        self.core.miss = T::ZERO;
        self.outgoing_axons
            .iter()
            .for_each(|a| self.core.miss += a.calculate_miss());
    }*/
}

impl<T> Nucleus<T> for HiddenCell<T>
where
    T: Debug,
{
    fn get_value(&self) -> &T {
        &self.core.value
    }
}

impl<T> Neuron<T> for HiddenCell<T>
where
    T: Float + Debug,
{
    fn get_miss(&self) -> &T {
        &self.core.miss
    }

    //////////////////////////////////////////////////////////////////////////
    // Forward propagation.
    //////////////////////////////////////////////////////////////////////////

    fn calculate_value(&mut self) {
        self.core.calculate_value();
    }

    //////////////////////////////////////////////////////////////////////////
    // Backward propagation.
    //////////////////////////////////////////////////////////////////////////

    /*fn calculate_miss(&mut self) {
        self.calculate_miss();
    }*/

    fn calculate_miss(&mut self) {
        self.core.miss = T::ZERO;
        self.outgoing_axons
            .iter()
            .for_each(|a| self.core.miss += a.calculate_miss());
    }

    fn calculate_weight(&mut self, rate: &T) {
        self.core.calculate_weight(rate);
    }
}

// Debugging.
/*impl<T> Debug for HiddenCell<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("HiddenCell")
            .field("core", &self.core)
            .field("outgoing_axons", &self.outgoing_axons)
            .finish()
    }
}*/
