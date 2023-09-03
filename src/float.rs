/// Float
pub trait FloatingPoint {
    type Float;

    fn to_real(&self) -> &Self {
        self
    }
}

enum Real<T> {
    F32(T),
    F64(T),
}

impl FloatingPoint for f32 {
    type Float = f32;

    // fn to_real(self) -> Self::Float {
    //     self
    // }
}

impl FloatingPoint for f64 {
    type Float = f64;

    // fn to_real(self) -> Self::Float {
    //     self
    // }
}
