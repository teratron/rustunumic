//! # Loss
//!
//!

use crate::float::Float;

pub(super) const LOSS_LIMIT: f64 = 1E-10;

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

pub(super) fn get_loss<T: Float>(value: &T, mode: &Loss) -> T {
    match mode {
        Loss::Avg => value.abs(),
        Loss::Arctan => value.atan().powi(2),
        Loss::MSE | Loss::RMSE | _ => value.powi(2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loss() {
        let value = 2.;

        assert_eq!(get_loss(&value, &Loss::MSE), 4.);
        assert_eq!(get_loss(&value, &Loss::RMSE), 4.);
        assert_eq!(get_loss(&value, &Loss::Avg), 2.);
        assert_eq!(get_loss(&value, &Loss::Arctan), value.atan().powi(2));
    }
}
