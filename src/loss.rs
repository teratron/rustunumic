//! # Loss
//!
//!

/// ## Loss mode
///
/// **List of mode:**
///
/// | Mode   | Description             |
/// |:-------|:------------------------|
/// | MSE    | Mean Squared Error      |
/// | RMSE   | Root Mean Squared Error |
/// | Arctan | Arctan Error            |
/// | Avg    | Average Error           |
///
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum Loss {
    /// Mean Squared Error.
    MSE,

    /// Root Mean Squared Error.
    RMSE,

    /// Arctan Error.
    Arctan,

    /// Average Error.
    Avg,
}

pub(super) fn get_loss<T>(value: &T, mode: &Loss) -> T {
    match mode {
        Loss::Avg => value.abs(),
        Loss::Arctan => value.atan().powi(2),
        Loss::MSE | Loss::RMSE | _ => value.powi(2),
    }
}
