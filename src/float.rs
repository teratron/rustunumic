//! # Float
//!
//! This module defines a `Float` trait to abstract over `f32` and `f64`,
//! allowing the library user to choose their desired floating-point precision.
//!
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::loss::LOSS_LIMIT;

/// Trait for generic floating-point operations.
///
/// This trait provides a common interface for `f32` and `f64`, enabling
/// type-agnostic numerical computations within the library.
pub trait Float
where
    Self: Sized
        + Copy
        + PartialOrd
        + PartialEq
        + Into<f64> // Allow conversion to f64 for interoperability
        + Mul<Output = Self>
        + MulAssign
        + Div<Output = Self>
        + DivAssign
        + Add<Output = Self>
        + AddAssign
        + Sub<Output = Self>
        + SubAssign
        + Neg<Output = Self>,
{
    // Associated constants for common float values
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const DEFAULT_RATE: Self;
    const LOSS_LIMIT: Self;

    /// Converts an `f64` value to `Self`.
    /// This is used when a literal `f64` needs to be converted to the generic float type.
    fn from_f64(x: f64) -> Self;

    /// Returns the absolute value of `self`.
    fn abs(self) -> Self;
    /// Raises `self` to the power of `n`.
    fn powi(self, n: i32) -> Self;
    /// Returns the square root of `self`.
    fn sqrt(self) -> Self;
    /// Returns `e` to the power of `self`.
    fn exp(self) -> Self;
    /// Returns the arctangent of `self`.
    fn atan(self) -> Self;
    /// Returns the hyperbolic tangent of `self`.
    fn tanh(self) -> Self;
    /// Returns the minimum of `self` and `other`.
    fn min(self, other: Self) -> Self;
    /// Returns the maximum of `self` and `other`.
    fn max(self, other: Self) -> Self;
}

// Global constants for default values, used in implementations.
// These are defined as f64 to avoid precision loss when casting to f32.
const ZERO: f64 = 0.0;
const ONE: f64 = 1.0;
const TWO: f64 = 2.0;
const DEFAULT_RATE: f64 = 0.3;

// f32 implementation of the Float trait.
impl Float for f32 {
    const ZERO: Self = ZERO as Self;
    const ONE: Self = ONE as Self;
    const TWO: Self = TWO as Self;
    const DEFAULT_RATE: Self = DEFAULT_RATE as Self;
    const LOSS_LIMIT: Self = LOSS_LIMIT as Self;

    fn from_f64(x: f64) -> Self {
        x as Self
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn exp(self) -> Self {
        self.exp()
    }

    fn atan(self) -> Self {
        self.atan()
    }

    fn tanh(self) -> Self {
        self.tanh()
    }

    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    fn max(self, other: Self) -> Self {
        self.max(other)
    }
}

// f64 implementation of the Float trait.
impl Float for f64 {
    const ZERO: Self = ZERO;
    const ONE: Self = ONE;
    const TWO: Self = TWO;
    const DEFAULT_RATE: Self = DEFAULT_RATE;
    const LOSS_LIMIT: Self = LOSS_LIMIT;

    fn from_f64(x: f64) -> Self {
        x
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn exp(self) -> Self {
        self.exp()
    }

    fn atan(self) -> Self {
        self.atan()
    }

    fn tanh(self) -> Self {
        self.tanh()
    }

    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    fn max(self, other: Self) -> Self {
        self.max(other)
    }
}
