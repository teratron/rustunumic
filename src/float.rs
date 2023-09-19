use std::fmt::Debug;

/// Float
pub trait Float<T>: Debug {
    type FloatType;

    fn type_name(&self) -> &'static str;
    //fn to_real(self) -> Self;
    /*fn to_real(&self: Float) -> &Self::Float {
        self
    }*/
    fn to_real(&self) -> &Self {
        &Self::from(self)
    }
}

impl<T> Float<T> for f32 {
    type FloatType = f32;

    fn type_name(&self) -> &'static str {
        "f32"
    }
    // fn to_real(self) -> Self {
    //     self
    // }
}

impl<T> Float<T> for f64 {
    type FloatType = f64;

    fn type_name(&self) -> &'static str {
        "f64"
    }
    // fn to_real(self) -> Self {
    //     f64::from(self)
    // }
}

// impl Into<T> for f64 {
//     fn into(self) -> T {
//         0.6
//     }
// }
