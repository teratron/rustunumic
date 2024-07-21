//! # Output Cell
//!
//!

//use std::fmt::Debug;

use std::ops::{Deref, DerefMut};

use crate::Float;

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
    pub(crate) fn new(target: &'a T) -> Self {
        Self {
            core: CoreCell::new(),
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
        //&self.core.value
        &self.value
    }
}

impl<T> Neuron<T> for OutputCell<'_, T>
where
    T: Float,
    /* + Debug*/
{
    fn get_miss(&self) -> &T {
        //&self.core.miss
        &self.miss
    }

    //////////////////////////////////////////////////////////////////////////
    // Forward propagation.
    //////////////////////////////////////////////////////////////////////////

    fn calculate_value(&mut self) {
        self.core.calculate_value();
        //self.core.miss = *self.target - self.core.value;
        self.miss = *self.target - self.value;
    }

    //////////////////////////////////////////////////////////////////////////
    // Backward propagation.
    //////////////////////////////////////////////////////////////////////////

    fn calculate_weight(&mut self, rate: &T) {
        self.core.calculate_weight(rate);
    }
}

impl<T> Deref for OutputCell<'_, T> {
    type Target = CoreCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl<T> DerefMut for OutputCell<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.core
    }
}

// Debugging.
/*impl<T> Debug for OutputCell<'_, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("OutputCell")
            .field("core", &self.core)
            .field("target", &self.target)
            .finish()
    }
}*/
