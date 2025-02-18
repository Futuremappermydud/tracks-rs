use std::any::Any;

use super::{
    operation::Operation,
    {Modifier, ModifierBase},
};
use crate::values::BaseValues;
use crate::values::base_provider_context::BaseProviderContext;
use glam::Vec4;

pub struct Vector4Modifier {
    raw_point: Option<Vec4>,
    values: Option<Vec<Box<dyn BaseValues>>>,
    modifiers: Vec<Box<dyn ModifierBase<Value = Vec4>>>,
    operation: Operation,
}

impl Vector4Modifier {
    pub fn new(
        point: Option<Vec4>,
        values: Option<Vec<Box<dyn BaseValues>>>,
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

    fn get_raw_point(&self) -> Vec4 {
        self.raw_point.unwrap_or(Vec4::ZERO)
    }

    fn translate(&self, values: &[f32]) -> Vec4 {
        Vec4::new(values[0], values[1], values[2], values[3])
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Modifier for Vector4Modifier {
    const VALUE_COUNT: usize = 4;
}
