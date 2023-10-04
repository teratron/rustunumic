//! # Loss
//!
//!

/// ## Loss mode
///
/// ### List of mode:
///
/// | Mode   | Description             |
/// |--------|-------------------------|
/// | MSE    | Mean Squared Error      |
/// | RMSE   | Root Mean Squared Error |
/// | Arctan | Arctan Error            |
/// | Avg    | Average Error           |
///
#[derive(Debug)]
pub enum Loss {
    /// MSE - Mean Squared Error.
    MSE,

    /// RMSE - Root Mean Squared Error.
    RMSE,

    /// Arctan - Arctan Error.
    Arctan,

    /// Avg - Average Error.
    Avg,
}
