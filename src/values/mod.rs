use crate::values::base_provider_context::BaseProviderContext;
use glam::{DVec2, DVec3, DVec4, Quat};
use serde_json::Value as JsonValue;
use std::{any::Any, sync::Arc};

pub mod base;
pub mod base_provider_context;
pub mod quat;
pub mod smooth;
pub mod smooth_rot;
pub mod r#static;
pub mod updatable;

pub trait ValueProvider {
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
    Vector2(DVec2),
    Vector3(DVec3),
    Vector4(DVec4),
    Quaternion(Quat),
}

pub trait UpdateableValues: ValueProvider {
    fn update(&mut self);
}

pub enum BaseValueProvider {
    Static(r#static::StaticValues),
    BaseProvider(base::BaseProviderValues),
    QuaternionProvider(quat::QuaternionProviderValues),
    PartialProvider(updatable::PartialProviderValues),
    SmoothProviders(smooth::SmoothProvidersValues),
    SmoothRotationProviders(smooth_rot::SmoothRotationProvidersValues),
}

impl ValueProvider for BaseValueProvider {
    fn values(&self, context: &BaseProviderContext) -> Value {
        let items = match self {
            BaseValueProvider::Static(v) => v.values(context),
            BaseValueProvider::BaseProvider(v) => v.values(context),
            BaseValueProvider::QuaternionProvider(v) => v.values(context),
            BaseValueProvider::PartialProvider(v) => v.values(context),
            BaseValueProvider::SmoothProviders(v) => v.values(context),
            BaseValueProvider::SmoothRotationProviders(v) => v.values(context),
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
fn close(
    result: &mut Vec<BaseValueProvider>,
    raw_values: Vec<&JsonValue>,
    open: usize,
    end: usize,
) {
    if end <= open {
        return;
    }

    let values: Vec<f32> = raw_values[open..end]
        .iter()
        .filter_map(|v| v.as_f64().map(|i| i as f32))
        .collect();

    result.push(Box::new(r#static::StaticValues::new(values)));
}
