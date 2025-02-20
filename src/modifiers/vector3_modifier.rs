use super::ModifierValues;
use super::{Modifier, ModifierBase, operation::Operation};
use crate::values::ValueProvider;
use crate::values::base_provider_context::BaseProviderContext;
use glam::Vec3;

pub type Vector3Values = ModifierValues<Vec3>;

pub struct Vector3Modifier {
    values: Vector3Values,
    modifiers: Vec<Modifier>,
    operation: Operation,
}

impl Vector3Modifier {
    pub fn new(point: Vector3Values, modifiers: Vec<Modifier>, operation: Operation) -> Self {
        Self {
            values: point,
            modifiers,
            operation,
        }
    }
}

impl ModifierBase for Vector3Modifier {
    type Value = Vec3;
    const VALUE_COUNT: usize = 3;

    fn get_point(&self, context: &BaseProviderContext) -> Vec3 {
        let original_point = match &self.values {
            Vector3Values::Static(s) => *s,
            Vector3Values::Dynamic(value_providers) => self.convert(&value_providers, context),
        };
        self.modifiers
            .iter()
            .fold(original_point, |acc, x| match x.get_operation() {
                Operation::Add => acc + x.get_vector3(context),
                Operation::Sub => acc - x.get_vector3(context),
                Operation::Mul => acc * x.get_vector3(context),
                Operation::Div => acc / x.get_vector3(context),
                Operation::None => x.get_vector3(context),
            })
    }

    fn get_raw_point(&self) -> Vec3 {
        self.values.as_static_values().copied().unwrap_or_default()
    }

    fn translate(&self, values: &[f32]) -> Vec3 {
        Vec3::new(values[0], values[1], values[2])
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
}
