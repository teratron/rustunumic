use crate::cell::Nucleus;

struct BiasCell;

impl Nucleus for BiasCell {
    fn get_value(&self) -> &f32 {
        &1.
    }
}