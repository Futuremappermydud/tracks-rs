use std::any::Any;

use crate::{
    easings::functions::Functions,
    modifiers::{ModifierBase, operation::Operation, vector3_modifier::Vector3Modifier},
    values::{BaseValues, base_provider_context::BaseProviderContext},
};
use glam::Vec3;

use super::BasePointData;

pub struct Vector3PointData {
    base_modifier: Vector3Modifier,
    easing: Functions,
    pub smooth: bool,
    time: f32,
}

impl Vector3PointData {
    pub fn new(
        point: Option<Vec3>,
        values: Option<Vec<Box<dyn BaseValues>>>,
        smooth: bool,
        time: f32,
        modifiers: Vec<Box<dyn ModifierBase<Value = Vec3>>>,
        easing: Functions,
    ) -> Self {
        Self {
            base_modifier: Vector3Modifier::new(point, values, modifiers, Operation::None),
            easing,
            smooth,
            time,
        }
    }
}

impl ModifierBase for Vector3PointData {
    type Value = Vec3;

    fn get_point(&self, context: &BaseProviderContext) -> Vec3 {
        self.base_modifier.get_point(context)
    }

    fn get_raw_point(&self) -> Vec3 {
        self.base_modifier.get_raw_point()
    }

    fn translate(&self, values: &[f32]) -> Vec3 {
        self.base_modifier.translate(values)
    }

    fn get_operation(&self) -> Operation {
        self.base_modifier.get_operation()
    }
}

impl BasePointData<Vec3> for Vector3PointData {
    fn get_easing(&self) -> Functions {
        self.easing.clone()
    }

    fn get_time(&self) -> f32 {
        self.time
    }

    fn has_base_provider(&self) -> bool {
        false
    }

    fn get_point(&self, context: &BaseProviderContext) -> Vec3 {
        <Self as ModifierBase>::get_point(self, context)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
