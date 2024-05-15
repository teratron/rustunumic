//! # Float
//!
//!

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::loss::LOSS_LIMIT;

const ZERO: f64 = 0.;
const ONE: f64 = 1.;
const TWO: f64 = 2.;
const DEFAULT_RATE: f64 = 0.3;

/// Float trait
pub trait Float
where
    Self: Sized
        + Copy
        + PartialOrd
        + PartialEq
        //+ From<f64>
        //+ Into<f64>
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
    type FloatType;

    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const DEFAULT_RATE: Self;
    const LOSS_LIMIT: Self;

    fn from(x: f64) -> Self;
    fn type_name(&self) -> &'static str;

    // Math functions.
    fn float_abs(&self) -> Self;
    fn float_powi(&self, n: i32) -> Self;
    fn float_sqrt(&self) -> Self;
    fn float_exp(&self) -> Self;
    fn float_atan(&self) -> Self;
    fn float_tanh(&self) -> Self;
    fn float_min(self, other: f64) -> Self;
    fn float_max(self, other: f64) -> Self;
}

// f32.
impl Float for f32 {
    type FloatType = f32;

    const ZERO: Self = ZERO as Self;
    const ONE: Self = ONE as Self;
    const TWO: Self = TWO as Self;
    const DEFAULT_RATE: Self = DEFAULT_RATE as Self;
    const LOSS_LIMIT: Self = LOSS_LIMIT as Self;

    fn from(x: f64) -> Self {
        x as Self::FloatType
    }

    fn type_name(&self) -> &'static str {
        "f32"
    }

    // Math functions.
    fn float_abs(&self) -> Self {
        self.abs()
    }

    fn float_powi(&self, n: i32) -> Self {
        self.powi(n)
    }

    fn float_sqrt(&self) -> Self {
        self.sqrt()
    }

    fn float_exp(&self) -> Self {
        self.exp()
    }

    fn float_atan(&self) -> Self {
        self.atan()
    }

    fn float_tanh(&self) -> Self {
        self.tanh()
    }

    fn float_min(self, other: f64) -> Self {
        self.min(other as Self::FloatType)
    }

    fn float_max(self, other: f64) -> Self {
        self.max(other as Self::FloatType)
    }
}

// f64.
impl Float for f64 {
    type FloatType = f64;

    const ZERO: Self = ZERO;
    const ONE: Self = ONE;
    const TWO: Self = TWO;
    const DEFAULT_RATE: Self = DEFAULT_RATE;
    const LOSS_LIMIT: Self = LOSS_LIMIT;

    fn from(x: f64) -> Self {
        x
    }

    fn type_name(&self) -> &'static str {
        "f64"
    }

    // Math functions.
    fn float_abs(&self) -> Self {
        self.abs()
    }

    fn float_powi(&self, n: i32) -> Self {
        self.powi(n)
    }

    fn float_sqrt(&self) -> Self {
        self.sqrt()
    }

    fn float_exp(&self) -> Self {
        self.exp()
    }

    fn float_atan(&self) -> Self {
        self.atan()
    }

    fn float_tanh(&self) -> Self {
        self.tanh()
    }

    fn float_min(self, other: f64) -> Self {
        self.min(other)
    }

    fn float_max(self, other: f64) -> Self {
        self.max(other)
    }
}
