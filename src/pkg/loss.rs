//! # Loss
//!
//!

/// ## Loss mode
///
/// ### List of mode:
///
/// | Code | Loss   | Description             |
/// |------|--------|-------------------------|
/// | 0    | MSE    | Mean Squared Error      |
/// | 1    | RMSE   | Root Mean Squared Error |
/// | 2    | ARCTAN | Arctan Error            |
/// | 3    | Avg    | Average Error           |
#[repr(u8)]
#[derive(Debug)]
pub enum Loss {
    /// MSE - Mean Squared Error.
    MSE,

    /// RMSE - Root Mean Squared Error.
    RMSE,

    /// ARCTAN - Arctan Error.
    ARCTAN,

    /// AVG - Average Error.
    Avg,
}
