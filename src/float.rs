/// Float
pub trait Float {
    type T;

    fn to_real(self) -> Self::T;
}

impl Float for f32 {
    type T = f32;

    fn to_real(self) -> Self::T {
        self as f32
    }
}

impl Float for f64 {
    type T = f64;

    fn to_real(self: <f64 as Float>::T) -> Self::T {
        self as f64
    }
}
