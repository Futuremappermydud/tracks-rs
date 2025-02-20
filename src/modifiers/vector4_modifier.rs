
use super::{Modifier, ModifierBase, operation::Operation};
use crate::values::ValueProvider;

use crate::values::base_provider_context::BaseProviderContext;
use glam::Vec4;

pub struct Vector4Modifier {
    raw_point: Option<Vec4>,
    values: Option<Vec<ValueProvider>>,
    modifiers: Vec<Box<dyn ModifierBase<Value = Vec4>>>,
    operation: Operation,
}

impl Vector4Modifier {
    pub fn new(
        point: Option<Vec4>,
        values: Option<Vec<ValueProvider>>,
        modifiers: Vec<Box<dyn ModifierBase<Value = Vec4>>>,
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

impl ModifierBase for Vector4Modifier {
    type Value = Vec4;

    fn get_point(&self, context: &BaseProviderContext) -> Vec4 {
        let original_point = self
            .raw_point
            .unwrap_or_else(|| self.convert(self.values.as_ref().unwrap(), context));
        let result = self
            .modifiers
            .iter()
            .fold(original_point, |acc, x| match x.get_operation() {
                Operation::Add => acc + x.get_vector4(context),
                Operation::Sub => acc - x.get_vector4(context),
                Operation::Mul => acc * x.get_vector4(context),
                Operation::Div => acc / x.get_vector4(context),
                Operation::None => x.get_vector4(context),
            });
        result
    }

    fn get_raw_point(&self) -> Vec4 {
        self.raw_point.unwrap_or(Vec4::ZERO)
    }

    fn translate(&self, values: &[f32]) -> Vec4 {
        Vec4::new(values[0], values[1], values[2], values[3])
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
}
