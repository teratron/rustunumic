//! # Loss
//!
//!

use super::Float;

pub(super) const LOSS_LIMIT: f64 = 1E-10;

pub mod arctan;
pub mod avg;
pub mod bce;
pub mod mae;
pub mod mape;
pub mod mse;
pub mod rmse; // Mean Absolute Percentage Error

/// Loss mode
///
/// **Note:**
///
/// - MSE is mean squared error.
/// - RMSE is root mean squared error.
/// - Arctan is arctan error.
/// - Avg is average error (Mean Absolute Error).
/// - MAE is Mean Absolute Error (equivalent to Avg).
/// - BCE is Binary Cross-Entropy (placeholder, requires API change).
/// - MAPE is Mean Absolute Percentage Error (placeholder, requires API change).
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum Loss {
    /// Mean Squared Error.
    MSE,

    /// Root Mean Squared Error.
    RMSE,

    /// Arctan Error.
    Arctan,

    /// Average Error (Mean Absolute Error).
    Avg,

    /// Mean Absolute Error (equivalent to Avg).
    MAE,

    /// Binary Cross-Entropy (placeholder).
    BCE,

    /// Mean Absolute Percentage Error (placeholder).
    MAPE,
}

impl Default for Loss {
    fn default() -> Self {
        Loss::MSE
    }
}

pub fn get_total_loss<T: Float>(misses: Vec<&T>, mode: &Loss) -> T {
    let mut loss = T::ZERO;
    let mut count = T::ZERO;
    misses.into_iter().for_each(|m| {
        loss += get_loss(m, mode);
        count += T::ONE;
    });
    if count > T::ONE {
        loss /= count;
    }
    if *mode == Loss::RMSE {
        loss = loss.sqrt();
    }
    loss
}

fn get_loss<T: Float>(value: &T, mode: &Loss) -> T {
    match mode {
        Loss::Avg => avg::calculate(value),
        Loss::Arctan => arctan::calculate(value),
        Loss::MSE => mse::calculate(value),
        Loss::RMSE => rmse::calculate(value), // RMSE uses the same calculation as MSE, square root is applied later
        Loss::MAE => mae::calculate(value),   // MAE is the same as Avg
        Loss::BCE => bce::calculate(value),   // BCE is a placeholder
        Loss::MAPE => mape::calculate(value, &T::ONE), // MAPE is a placeholder, needs target
    }
}
