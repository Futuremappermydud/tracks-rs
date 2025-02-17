use std::any::Any;

use crate::{
    easings::functions::Functions,
    modifiers::{ModifierBase, float_modifier::FloatModifier, operation::Operation},
    values::BaseValues,
};

use super::BasePointData;

pub struct FloatPointData {
    base_modifier: FloatModifier,
    easing: Functions,
    time: f32,
}

impl FloatPointData {
    pub fn new(
        point: Option<f32>,
        values: Option<Vec<Box<dyn BaseValues>>>,
        time: f32,
        modifiers: Vec<Box<dyn ModifierBase<Value = f32>>>,
        easing: Functions,
    ) -> Self {
        Self {
            base_modifier: FloatModifier::new(point, values, modifiers, Operation::None),
            easing,
            time,
        }
    }
}

impl ModifierBase for FloatPointData {
    type Value = f32;

    fn get_point(&self) -> f32 {
        self.base_modifier.get_point()
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

    fn get_point(&self) -> f32 {
        <Self as ModifierBase>::get_point(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
