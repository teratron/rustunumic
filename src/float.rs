use std::fmt::Debug;

/// Float
pub trait Float: Debug {
    type FloatType;
    //const INITIAL_VALUE: Self;

    fn type_name(&self) -> &'static str;
    fn as_float(v: f64) -> Self::FloatType; /*{
        v as Self::FloatType
    }*/
    //fn to_real(self) -> Self;
    /*fn to_real(&self: Float) -> &Self::Float {
        self
    }*/
    // fn to_real(v: f64) -> Self::FloatType {
    //     v as Self::FloatType
    // }
}

impl Float for f32 {
    type FloatType = f32;
    //const INITIAL_VALUE: Self = 1.0;

    fn type_name(&self) -> &'static str {
        "f32"
    }

    fn as_float(v: f64) -> Self::FloatType {
        v as f32
    }
    // fn to_real(self) -> Self {
    //     self
    // }
}

impl Float for f64 {
    type FloatType = f64;
    //const INITIAL_VALUE: Self = 1.0;

    fn type_name(&self) -> &'static str {
        "f64"
    }

    fn as_float(v: f64) -> Self::FloatType {
        v
    }
    // fn to_real(self) -> Self {
    //     f64::from(self)
    // }
}
/*impl<U: Float> Into<U> for f32 {
    fn into(self) -> U {
        self as Float::FloatType
    }
}*/
// impl Into<T> for f64 {
//     fn into(self) -> T {
//         0.6
//     }
// }

// struct Wrapper<T>(Vec<T>);
//
// impl<T> From<Wrapper<T>> for Vec<T> {
//     fn from(w: Wrapper<T>) -> Vec<T> {
//         w.0
//     }
// }
//
// impl<T> Into<Vec<T>> for Wrapper<T> {
//     fn into(self) -> Vec<T> {
//         self.0
//     }
// }
