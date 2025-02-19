pub mod float_modifier;
pub mod operation;
pub mod quaternion_modifier;
pub mod vector3_modifier;
pub mod vector4_modifier;

use crate::modifiers::operation::Operation;
use crate::values::base_provider_context::BaseProviderContext;
use crate::values::{AbstractValueProvider, ValueProvider};

pub trait ModifierBase {
    type Value;
    fn get_point(&self, context: &BaseProviderContext) -> Self::Value;
    fn get_raw_point(&self) -> Self::Value;
    fn translate(&self, values: &[f32]) -> Self::Value;
    fn get_operation(&self) -> Operation;
}

pub trait Modifier: ModifierBase {
    const VALUE_COUNT: usize;

    fn fill_values(&self, ivals: &[ValueProvider], context: &BaseProviderContext) -> Vec<f32> {
        let mut values = Vec::with_capacity(Self::VALUE_COUNT);
        for value in ivals {
            for v in value.values(context) {
                if values.len() < Self::VALUE_COUNT {
                    values.push(v);
                } else {
                    return values;
                }
            }
        }
        values
    }

    fn convert(&self, ivals: &[ValueProvider], context: &BaseProviderContext) -> Self::Value {
        let values = self.fill_values(ivals, context);
        self.translate(&values)
    }
}
