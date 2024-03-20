use crate::cell::CoreTrait;

struct InputCell(f32);

impl CoreTrait for InputCell {
    fn get_value(&self) -> &f32 {
        &self.0
    }
}