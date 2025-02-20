use core::slice::SlicePattern;
use std::{borrow::Cow, cell::RefCell};

use super::{Modifier, ModifierBase, ModifierValues, operation::Operation};
use crate::values::{
    AbstractValueProvider, ValueProvider, base_provider_context::BaseProviderContext,
};
use glam::{EulerRot, Quat, Vec3};

pub enum QuaternionValues {
    StaticVec(Vec3),
    StaticQuat(Quat),
    Dynamic(Vec<ValueProvider>),
}

pub struct QuaternionModifier {
    values: QuaternionValues,
    modifiers: Vec<Modifier>,
    operation: Operation,
}

impl QuaternionModifier {
    pub fn new(point: QuaternionValues, modifiers: Vec<Modifier>, operation: Operation) -> Self {
        Self {
            values: point,
            modifiers,
            operation,
        }
    }

    fn translate_euler<'a>(
        &self,
        values: &'a Vec<ValueProvider>,
        context: &BaseProviderContext,
    ) -> Vec3 {
        let mut vec3 = Vec3::ZERO;

        values
            .iter()
            .flat_map(|x| x.values(context).iter().copied().collect::<Vec<_>>())
            .take(Self::VALUE_COUNT)
            .enumerate()
            .for_each(|(i, v)| vec3[i] = v);

        vec3
    }

    pub fn get_vector_point(&self, context: &BaseProviderContext) -> Vec3 {
        let original_point = match &self.values {
            QuaternionValues::StaticQuat(s) => {
                // returns radians
                let (x, y, z) = s.clone().to_euler(EulerRot::ZXY);

                Vec3::new(x.to_degrees(), y.to_degrees(), z.to_degrees())
            }
            QuaternionValues::StaticVec(s) => *s,
            QuaternionValues::Dynamic(value_providers) => {
                self.translate_euler(&value_providers, context)
            }
        };
        self.modifiers.iter().fold(original_point, |acc, x| {
            let Modifier::Quaternion(quat_point) = x else {
                panic!("Invalid modifier type");
            };
            match x.get_operation() {
                Operation::Add => acc + quat_point.get_vector_point(context),
                Operation::Sub => acc - quat_point.get_vector_point(context),
                Operation::Mul => acc * quat_point.get_vector_point(context),
                Operation::Div => acc / quat_point.get_vector_point(context),
                Operation::None => quat_point.get_vector_point(context),
            }
        })
    }
}

impl ModifierBase for QuaternionModifier {
    type Value = Quat;
    const VALUE_COUNT: usize = 3;

    fn get_point(&self, context: &BaseProviderContext) -> Quat {
        if self.modifiers.len() > 0 {
            self.get_raw_point()
        } else {
            let vector_point = self.get_vector_point(context);
            Quat::from_euler(
                EulerRot::XYZ,
                vector_point.x.to_radians(),
                vector_point.y.to_radians(),
                vector_point.z.to_radians(),
            )
        }
    }

    fn get_raw_point(&self) -> Quat {
        match self.values {
            QuaternionValues::StaticVec(s) => Quat::from_euler(
                EulerRot::XYZ,
                s.x.to_radians(),
                s.y.to_radians(),
                s.z.to_radians(),
            ),
            QuaternionValues::StaticQuat(s) => s,
            _ => Quat::IDENTITY,
        }
    }

    fn translate(&self, values: &[f32]) -> Quat {
        Quat::from_euler(
            EulerRot::ZXY,
            values[0].to_radians(),
            values[1].to_radians(),
            values[2].to_radians(),
        )
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
}
