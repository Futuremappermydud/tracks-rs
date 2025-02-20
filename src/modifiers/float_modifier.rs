use super::{Modifier, ModifierBase, operation::Operation};
use crate::values::ValueProvider;
use crate::values::base_provider_context::BaseProviderContext;

pub struct FloatModifier {
    raw_point: Option<f32>,
    values: Option<Vec<ValueProvider>>,
    modifiers: Vec<Box<Modifier>>,
    operation: Operation,
}

impl FloatModifier {
    pub fn new(
        point: Option<f32>,
        values: Option<Vec<ValueProvider>>,
        modifiers: Vec<Box<Modifier>>,
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

impl ModifierBase for FloatModifier {
    type Value = f32;
    const VALUE_COUNT: usize = 1;

    fn get_point(&self, context: &BaseProviderContext) -> f32 {
        let original_point = self
            .raw_point
            .unwrap_or_else(|| self.convert(self.values.as_ref().unwrap(), context));
        self.modifiers
            .iter()
            .fold(original_point, |acc, x| match x.get_operation() {
                Operation::Add => acc + x.get_float(context),
                Operation::Sub => acc - x.get_float(context),
                Operation::Mul => acc * x.get_float(context),
                Operation::Div => acc / x.get_float(context),
                Operation::None => x.get_float(context),
            })
    }

    fn get_raw_point(&self) -> f32 {
        self.raw_point.unwrap_or(0.0)
    }

    fn translate(&self, values: &[f32]) -> f32 {
        values[0]
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
}