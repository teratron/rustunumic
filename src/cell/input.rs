use crate::cell::Nucleus;

struct InputCell(f32);

impl Nucleus for InputCell {
    fn get_value(&self) -> &f32 {
        &self.0
    }
}