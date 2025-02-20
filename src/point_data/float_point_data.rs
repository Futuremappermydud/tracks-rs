use crate::{
    easings::functions::Functions,
    modifiers::{float_modifier::{FloatModifier, FloatValues}, operation::Operation, vector3_modifier::Vector3Values, Modifier, ModifierBase},
    values::{base_provider_context::BaseProviderContext, ValueProvider},
};

use super::BasePointData;

pub struct FloatPointData {
    base_modifier: FloatModifier,
    easing: Functions,
    time: f32,
}

impl FloatPointData {
    pub fn new(
        point: FloatValues,
        time: f32,
        modifiers: Vec<Modifier>,
        easing: Functions,
    ) -> Self {
        Self {
            base_modifier: FloatModifier::new(point, modifiers, Operation::None),
            easing,
            time,
        }
    }
}

impl ModifierBase for FloatPointData {
    type Value = f32;
    const VALUE_COUNT: usize = 1;

    fn get_point(&self, context: &BaseProviderContext) -> f32 {
        self.base_modifier.get_point(context)
    }

    fn get_raw_point(&self) -> f32 {
        self.base_modifier.get_raw_point()
    }

    fn translate(&self, values: &[f32]) -> f32 {
        self.base_modifier.translate(values)
    }

    fn get_operation(&self) -> Operation {
        self.base_modifier.get_operation()
    }
}

impl BasePointData<f32> for FloatPointData {
    fn get_easing(&self) -> Functions {
        self.easing.clone()
    }

    fn get_time(&self) -> f32 {
        self.time
    }
    fn has_base_provider(&self) -> bool {
        false
    }

    fn get_point(&self, context: &BaseProviderContext) -> f32 {
        <Self as ModifierBase>::get_point(self, context)
    }
}