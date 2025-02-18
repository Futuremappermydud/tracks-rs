use std::any::Any;

use crate::{
    easings::functions::Functions,
    modifiers::{
        ModifierBase, operation::Operation, quaternion_modifier::QuaternionModifier,
        vector3_modifier::Vector3Modifier,
    },
    values::{BaseValues, base_provider_context::BaseProviderContext},
};
use glam::{Quat, Vec3};

use super::BasePointData;

pub struct QuaternionPointData {
    base_modifier: QuaternionModifier,
    easing: Functions,
    time: f32,
}

impl QuaternionPointData {
    pub fn new(
        point: Option<Quat>,
        vector_point: Option<Vec3>,
        values: Option<Vec<Box<dyn BaseValues>>>,
        time: f32,
        modifiers: Vec<Box<dyn ModifierBase<Value = Quat>>>,
        easing: Functions,
    ) -> Self {
        Self {
            base_modifier: QuaternionModifier::new(
                point,
                vector_point,
                values,
                modifiers,
                Operation::None,
            ),
            easing,
            time,
        }
    }
}

impl ModifierBase for QuaternionPointData {
    type Value = Quat;

    fn get_point(&self, context: &BaseProviderContext) -> Quat {
        self.base_modifier.get_point(context)
    }

    fn get_raw_point(&self) -> Quat {
        self.base_modifier.get_raw_point()
    }

    fn translate(&self, values: &[f32]) -> Quat {
        self.base_modifier.translate(values)
    }

    fn get_operation(&self) -> Operation {
        self.base_modifier.get_operation()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl BasePointData<Quat> for QuaternionPointData {
    fn get_easing(&self) -> Functions {
        self.easing.clone()
    }

    fn get_time(&self) -> f32 {
        self.time
    }

    fn has_base_provider(&self) -> bool {
        false
    }

    fn get_point(&self, context: &BaseProviderContext) -> Quat {
        <Self as ModifierBase>::get_point(self, context)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
