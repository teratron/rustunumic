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
    /// Mean Squared Error.
    MSE,

    /// Root Mean Squared Error.
    RMSE,

    /// Arctan Error.
    Arctan,

    /// Average Error.
    Avg,
}
