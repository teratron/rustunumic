use std::fmt::Debug;

/// Float
pub trait Float: Debug {
    type FloatType;

    fn type_name(&self) -> &'static str;
    //fn to_real(self) -> Self;
    /*fn to_real(&self: Float) -> &Self::Float {
        self
    }*/
}

impl Float for f32 {
    type FloatType = f32;

    fn type_name(&self) -> &'static str {
        "f32"
    }
    // fn to_real(self) -> Self {
    //     self
    // }
}

impl Float for f64 {
    type FloatType = f32;

    fn type_name(&self) -> &'static str {
        "f64"
    }
    // fn to_real(self) -> Self::FloatType {
    //     let v = self as f32;
    //     v
    // }
}

// impl Into<T> for f64 {
//     fn into(self) -> T {
//         0.6
//     }
// }
