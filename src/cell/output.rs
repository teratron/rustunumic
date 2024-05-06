//! # Output Cell
//!
//!

//use std::fmt::Debug;

use crate::{Activation, Float};

use super::{CoreCell, Neuron, Nucleus};

//#[derive(Debug)]
pub(crate) struct OutputCell<'a, T> {
    /// Core cell.
    core: CoreCell<T>,

    /// Target data.
    target: &'a T,
}

impl<'a, T> OutputCell<'a, T>
where
    T: Float,
{
    pub(crate) fn new(activation_mode: Activation, target: &'a T) -> Self {
        Self {
            core: CoreCell::new(activation_mode),
            target,
        }
    }

    pub(crate) fn set_target(&mut self, value: &'a T) {
        self.target = value;
    }
}

impl<T> Nucleus<T> for OutputCell<'_, T>
/*where
T: Debug,*/
{
    fn get_value(&self) -> &T {
        &self.core.value
    }
}

impl<T> Neuron<T> for OutputCell<'_, T>
where
    T: Float, /* + Debug*/
{
    fn get_miss(&self) -> &T {
        &self.core.miss
    }

    //////////////////////////////////////////////////////////////////////////
    // Forward propagation.
    //////////////////////////////////////////////////////////////////////////

    fn calculate_value(&mut self) {
        self.core.calculate_value();
        //self.calculate_miss();
        self.core.miss = *self.target - self.core.value;
    }

    //////////////////////////////////////////////////////////////////////////
    // Backward propagation.
    //////////////////////////////////////////////////////////////////////////

    /*fn calculate_miss(&mut self) {
        self.core.miss = *self.target - self.core.value;
    }*/

    fn calculate_weight(&mut self, rate: &T) {
        self.core.calculate_weight(rate);
    }
}
