//use std::fmt::Debug;

const ZERO: f64 = 0.;
const DEFAULT_RATE: f64 = 0.3;

/// Float trait
pub trait Float {
    //:Debug
    type FloatType;

    const ZERO: Self = ZERO;
    const DEFAULT_RATE: Self::FloatType = DEFAULT_RATE;

    fn type_name(&self) -> &'static str;
}

/*impl<T:Debug> Float for T {
    type FloatType = T;

    fn type_name(&self) -> &'static str {
        ""
    }
}*/

impl Float for f32 {
    type FloatType = f32;

    const ZERO: Self = ZERO as Self;
    const DEFAULT_RATE: Self = DEFAULT_RATE as Self;

    fn type_name(&self) -> &'static str {
        "f32"
    }
}

impl Float for f64 {
    type FloatType = f64;

    //const ZERO: Self = ZERO;
    //const DEFAULT_RATE: Self::FloatType = DEFAULT_RATE;

    fn type_name(&self) -> &'static str {
        "f64"
    }
}
