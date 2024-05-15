//! TanH
//!
//!

use super::Float;

pub(super) fn activation<T: Float>(value: T) -> T {
    // let v = (T::from(2.) * value).float_exp();
    // (v - T::ONE) / (v + T::ONE)
    value.float_tanh()
}

pub(super) fn derivative<T: Float>(value: T) -> T {
    //T::from(1.) - value.float_powi(2)
    T::ONE - value.float_min(1.).float_max(-1.).float_powi(2)
}
