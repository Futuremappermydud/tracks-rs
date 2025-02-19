use crate::values::base_provider_context::BaseProviderContext;
use glam::{DVec2, DVec3, DVec4, Quat, Vec2, Vec3, Vec4};
use serde_json::Value as JsonValue;
use std::{any::Any, sync::Arc};

pub mod base;
pub mod base_provider_context;
pub mod quat;
pub mod smooth;
pub mod smooth_rot;
pub mod r#static;
pub mod updatable;

pub trait AbstractValueProvider {
    fn values(&self, context: &BaseProviderContext) -> Value;

    fn as_float(&self, context: &BaseProviderContext) -> Option<f32> {
        match self.values(context) {
            Value::Float(v) => Some(v),
            _ => None,
        }
    }

    fn as_vector2(&self, context: &BaseProviderContext) -> Option<DVec2> {
        match self.values(context) {
            Value::Vector2(v) => Some(v),
            _ => None,
        }
    }

    fn as_vector3(&self, context: &BaseProviderContext) -> Option<DVec3> {
        match self.values(context) {
            Value::Vector3(v) => Some(v),
            _ => None,
        }
    }

    fn as_vector4(&self, context: &BaseProviderContext) -> Option<DVec4> {
        match self.values(context) {
            Value::Vector4(v) => Some(v),
            _ => None,
        }
    }

    fn as_quaternion(&self, context: &BaseProviderContext) -> Option<Quat> {
        match self.values(context) {
            Value::Quaternion(v) => Some(v),
            _ => None,
        }
    }
}

pub enum Value {
    Float(f32),
    Vector2(Vec2),
    Vector3(Vec3),
    Vector4(Vec4),
    Quaternion(Quat),
}

impl IntoIterator for Value {
    type Item = f32;
    type IntoIter = Box<dyn Iterator<Item = f32>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Value::Float(v) => Box::new([v].into_iter()),
            Value::Vector2(v) => Box::new([v.x, v.y].into_iter()),
            Value::Vector3(v) => Box::new([v.x, v.y, v.z].into_iter()),
            Value::Vector4(v) => Box::new([v.x, v.y, v.z, v.w].into_iter()),
            Value::Quaternion(v) => Box::new([v.x, v.y, v.z, v.w].into_iter()),
        }
    }
}

pub trait UpdateableValues: AbstractValueProvider {
    fn update(&mut self);
}

pub enum ValueProvider {
    Static(r#static::StaticValues),
    BaseProvider(base::BaseProviderValues),
    QuaternionProvider(quat::QuaternionProviderValues),
    PartialProvider(updatable::PartialProviderValues),
    SmoothProviders(smooth::SmoothProvidersValues),
    SmoothRotationProviders(smooth_rot::SmoothRotationProvidersValues),
}

impl AbstractValueProvider for ValueProvider {
    fn values(&self, context: &BaseProviderContext) -> Value {
        let items = match self {
            ValueProvider::Static(v) => v.values(context),
            ValueProvider::BaseProvider(v) => v.values(context),
            ValueProvider::QuaternionProvider(v) => v.values(context),
            ValueProvider::PartialProvider(v) => v.values(context),
            ValueProvider::SmoothProviders(v) => v.values(context),
            ValueProvider::SmoothRotationProviders(v) => v.values(context),
        };
        items
    }
}

// Helper function for linear interpolation
fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t.clamp(0.0, 1.0)
}

// Values deserialization
#[cfg(feature = "json")]
pub fn deserialize_values(
    value: &[&JsonValue],
    _context: &BaseProviderContext,
) -> Vec<BaseProviderContext> {
    let mut result = Vec::new();
    let mut start = 0;

    for (i, v) in value.iter().enumerate() {
        if v.is_string() {
            close(&mut result, value.to_vec(), start, i);
            start = i + 1;
            let base = v.as_str().unwrap().to_string();
            result.push(Box::new(base::BaseProviderValues::new(base.clone())));
        }
    }

    close(&mut result, value.to_vec(), start, value.len());
    result
}

#[cfg(feature = "json")]
fn close(result: &mut Vec<ValueProvider>, raw_values: Vec<&JsonValue>, open: usize, end: usize) {
    if end <= open {
        return;
    }

    let values: Vec<f32> = raw_values[open..end]
        .iter()
        .filter_map(|v| v.as_f64().map(|i| i as f32))
        .collect();

    result.push(Box::new(r#static::StaticValues::new(values)));
}
