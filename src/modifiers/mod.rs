pub mod float_modifier;
pub mod operation;

use crate::modifiers::operation::Operation;
use crate::values::BaseValues;

pub trait ModifierBase {
    type Value;
    fn get_point(&self) -> Self::Value;
    fn get_raw_point(&self) -> Self::Value;
    fn translate(&self, values: &[f32]) -> Self::Value;
    fn get_operation(&self) -> Operation;
}

pub trait Modifier: ModifierBase {
    const VALUE_COUNT: usize;

    fn fill_values(&self, ivals: &[Box<dyn BaseValues>]) -> Vec<f32> {
        let mut values = Vec::with_capacity(Self::VALUE_COUNT);
        for value in ivals {
            for &v in value.values() {
                if values.len() < Self::VALUE_COUNT {
                    values.push(v);
                } else {
                    return values;
                }
            }
        }
        values
    }

    fn convert(&self, ivals: &[Box<dyn BaseValues>]) -> Self::Value {
        let values = self.fill_values(ivals);
        self.translate(&values)
    }
}
