use super::{Modifier, ModifierBase, operation::Operation};
use crate::values::ValueProvider;

use crate::values::base_provider_context::BaseProviderContext;
use glam::Vec3;

pub struct Vector3Modifier {
    raw_point: Option<Vec3>,
    values: Option<Vec<ValueProvider>>,
    modifiers: Vec<Box<dyn ModifierBase<Value = Vec3>>>,
    operation: Operation,
}

impl Vector3Modifier {
    pub fn new(
        point: Option<Vec3>,
        values: Option<Vec<ValueProvider>>,
        modifiers: Vec<Box<dyn ModifierBase<Value = Vec3>>>,
        operation: Operation,
    ) -> Self {
        Self {
            raw_point: point,
            values,
            modifiers,
            operation,
        }
    }
}

impl ModifierBase for Vector3Modifier {
    type Value = Vec3;

    fn get_point(&self, context: &BaseProviderContext) -> Vec3 {
        let original_point = self
            .raw_point
            .unwrap_or_else(|| self.convert(self.values.as_ref().unwrap(), context));
        self.modifiers
            .iter()
            .fold(original_point, |acc, x| match x.get_operation() {
                Operation::Add => acc + x.get_point(context),
                Operation::Sub => acc - x.get_point(context),
                Operation::Mul => acc * x.get_point(context),
                Operation::Div => acc / x.get_point(context),
                Operation::None => x.get_point(context),
            })
    }

    fn get_raw_point(&self) -> Vec3 {
        self.raw_point.unwrap_or(Vec3::ZERO)
    }

    fn translate(&self, values: &[f32]) -> Vec3 {
        Vec3::new(values[0], values[1], values[2])
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
}
