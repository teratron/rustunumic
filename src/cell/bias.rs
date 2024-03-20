use crate::cell::CoreTrait;

struct BiasCell;

impl CoreTrait for BiasCell {
    fn get_value(&self) -> &f32 {
        &1.
    }
}