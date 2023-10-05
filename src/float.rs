use std::fmt::Debug;

/// Float trait
pub trait Float: Debug {
    type FloatType;
    const INITIAL_VALUE: Self;

    fn type_name(&self) -> &'static str;
}

impl Float for f32 {
    type FloatType = f32;
    const INITIAL_VALUE: Self = 1.0;

    fn type_name(&self) -> &'static str {
        "f32"
    }
}

impl Float for f64 {
    type FloatType = f64;
    const INITIAL_VALUE: Self = 1.0;

    fn type_name(&self) -> &'static str {
        "f64"
    }
}
