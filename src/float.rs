//use std::fmt::Debug;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Sub};

const ZERO: f64 = 0.;
const ONE: f64 = 1.;
const TWO: f64 = 2.;
const DEFAULT_RATE: f64 = 0.3;
const DEFAULT_RAND: f64 = 0.5;

//trait FloatAsWeNeedIt: num::Float + 'static {}

/// Float trait
pub trait Float
where
    Self: Sized
        + Copy
        + PartialOrd
        + Mul<Output = Self>
        + Div<Output = Self>
        + DivAssign
        + Add<Output = Self>
        + AddAssign
        + Sub<Output = Self>
        + Neg<Output = Self>,
{
    type FloatType;

    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const DEFAULT_RATE: Self;
    const DEFAULT_RAND: Self;

    fn type_name(&self) -> &'static str;

    // Math functions.
    fn abs(&self) -> Self {
        self.abs()
    }

    fn powi(&self, n: i32) -> Self {
        self.powi(n)
    }

    fn sqrt(&self) -> Self {
        self.sqrt()
    }

    fn exp(&self) -> Self {
        self.exp()
    }

    fn atan(&self) -> Self {
        self.atan()
    }

    fn from(x: f64) -> Self;
}

// f32.
impl Float for f32 {
    type FloatType = f32;

    const ZERO: Self = ZERO as Self;
    const ONE: Self = ONE as Self;
    const TWO: Self = TWO as Self;
    const DEFAULT_RATE: Self = DEFAULT_RATE as Self;
    const DEFAULT_RAND: Self = DEFAULT_RAND as Self;

    fn type_name(&self) -> &'static str {
        "f32"
    }

    fn from(x: f64) -> Self {
        x as Self::FloatType
    }
}

// f64.
impl Float for f64 {
    type FloatType = f64;

    const ZERO: Self = ZERO;
    const ONE: Self = ONE;
    const TWO: Self = TWO;
    const DEFAULT_RATE: Self::FloatType = DEFAULT_RATE;
    const DEFAULT_RAND: Self::FloatType = DEFAULT_RAND;

    fn type_name(&self) -> &'static str {
        "f64"
    }

    fn from(x: f64) -> Self {
        x
    }
}
