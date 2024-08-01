//! # Hidden Cell
//!
//!

//use std::fmt::Debug;
//use std::ops::{Deref, DerefMut};

use crate::axon::AxonBundle;
use crate::Float;

use super::{CoreCell, Neuron, Nucleus};

//#[derive(Debug)]
pub(crate) struct HiddenCell<T> {
    /// Core cell.
    core: CoreCell<T>,

    /// Outgoing axons.
    outgoing_axons: AxonBundle<T>,
}

impl<T> HiddenCell<T>
where
    T: Float,
{
    pub(crate) fn new() -> Self {
        Self {
            core: CoreCell::new(),
            outgoing_axons: Vec::new(),
        }
    }

    pub(crate) fn calculate_miss(&mut self) {
        self.core.miss = T::ZERO;
        self.outgoing_axons
            .iter()
            .for_each(|a| self.core.miss += a.calculate_miss());
    }
}

impl<T> Nucleus<T> for HiddenCell<T>
/*where
T: Debug,*/
{
    fn get_value(&self) -> &T {
        &self.core.value
    }
}

impl<T> Neuron<T> for HiddenCell<T>
where
    T: Float,
    /* + Debug*/
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

    fn calculate_weight(&mut self, rate: &T) {
        self.core.calculate_weight(rate);
    }
}

/*impl<T> Deref for HiddenCell<T> {
    type Target = CoreCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl<T> DerefMut for HiddenCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.core
    }
}*/

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
