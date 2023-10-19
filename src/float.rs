use std::fmt::Debug;

const ABC: f64 = 0.3;

/// Float trait
pub trait Float: Debug {
    type FloatType;
    const DEFAULT_RATE: Self;

    fn type_name(&self) -> &'static str;
}

impl Float for f32 {
    type FloatType = f32;
    const DEFAULT_RATE: Self = ABC as Self;

    fn type_name(&self) -> &'static str {
        "f32"
    }
}

impl Float for f64 {
    type FloatType = f64;
    const DEFAULT_RATE: Self::FloatType = ABC;

    fn type_name(&self) -> &'static str {
        "f64"
    }
}
