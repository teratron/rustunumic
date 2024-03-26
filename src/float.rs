//use std::fmt::Debug;

const ZERO: f64 = 0.;
const DEFAULT_RATE: f64 = 0.3;

//trait FloatAsWeNeedIt: num::Float + 'static {}

/// Float trait
pub trait Float: PartialEq + PartialOrd {
    //: PartialEq + PartialOrd + Debug
    type FloatType;
    const ZERO: Self;
    const DEFAULT_RATE: Self;

    fn type_name(&self) -> &'static str;

    // Math functions.
    fn abs(&self) -> Self;
    fn powi(&self, n: i32) -> Self;
    fn sqrt(&self) -> Self;
    fn exp(&self) -> Self;
    fn atan(&self) -> Self;
}

// f32.
impl Float for f32 {
    type FloatType = f32;
    const ZERO: Self = ZERO as Self;
    const DEFAULT_RATE: Self = DEFAULT_RATE as Self;

    fn type_name(&self) -> &'static str {
        "f32"
    }

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
}

// f64.
impl Float for f64 {
    type FloatType = f64;
    const ZERO: Self = ZERO;
    const DEFAULT_RATE: Self::FloatType = DEFAULT_RATE;

    fn type_name(&self) -> &'static str {
        "f64"
    }

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
}

impl Float for &f32 {
    type FloatType = Self;
    const ZERO: Self = &0.;
    const DEFAULT_RATE: Self = &0.3;

    fn type_name(&self) -> &'static str {
        todo!()
    }

    fn abs(&self) -> Self {
        todo!()
    }

    fn powi(&self, n: i32) -> Self {
        todo!()
    }

    fn sqrt(&self) -> Self {
        todo!()
    }

    fn exp(&self) -> Self {
        todo!()
    }

    fn atan(&self) -> Self {
        todo!()
    }
}

impl Float for &f64 {
    type FloatType = Self;
    const ZERO: Self = &0.;
    const DEFAULT_RATE: Self = &0.3;

    fn type_name(&self) -> &'static str {
        todo!()
    }

    fn abs(&self) -> Self {
        todo!()
    }

    fn powi(&self, n: i32) -> Self {
        todo!()
    }

    fn sqrt(&self) -> Self {
        todo!()
    }

    fn exp(&self) -> Self {
        todo!()
    }

    fn atan(&self) -> Self {
        todo!()
    }
}
