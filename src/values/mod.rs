use crate::values::base_provider_context::BaseProviderContext;
use glam::{DVec2, DVec3, DVec4};
use serde_json::Value as JsonValue;
use std::{
    any::Any,
    ops::{Add, Div, Index, IndexMut, Mul},
    sync::Arc,
};

pub mod base;
pub mod base_provider_context;
pub mod quat;
pub mod smooth;
pub mod smooth_rot;
pub mod r#static;
pub mod updatable;
pub mod value;

pub trait AbstractValueProvider {
    fn values(&self, context: &BaseProviderContext) -> value::Value;
}

// scalar ops

impl Mul<f32> for value::Value {
    type Output = value::Value;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            value::Value::Float(v) => value::Value::Float(v * rhs),
            value::Value::Vector2(v) => value::Value::Vector2(v * rhs),
            value::Value::Vector3(v) => value::Value::Vector3(v * rhs),
            value::Value::Vector4(v) => value::Value::Vector4(v * rhs),
            value::Value::Quaternion(v) => value::Value::Quaternion(v * rhs),
        }
    }
}

impl Div<f32> for value::Value {
    type Output = value::Value;

    fn div(self, rhs: f32) -> Self::Output {
        match self {
            value::Value::Float(v) => value::Value::Float(v / rhs),
            value::Value::Vector2(v) => value::Value::Vector2(v / rhs),
            value::Value::Vector3(v) => value::Value::Vector3(v / rhs),
            value::Value::Vector4(v) => value::Value::Vector4(v / rhs),
            value::Value::Quaternion(v) => value::Value::Quaternion(v / rhs),
        }
    }
}

impl Index<usize> for value::Value {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            value::Value::Float(f) => f,
            value::Value::Vector2(v) => &v[index],
            value::Value::Vector3(v) => &v[index],
            value::Value::Vector4(v) => &v[index],
            value::Value::Quaternion(v) => match index {
                0 => &v.x,
                1 => &v.y,
                2 => &v.z,
                3 => &v.w,
                _ => panic!("Invalid index for Quaternion"),
            },
        }
    }
}
impl IndexMut<usize> for value::Value {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            value::Value::Float(f) => f,
            value::Value::Vector2(v) => &mut v[index],
            value::Value::Vector3(v) => &mut v[index],
            value::Value::Vector4(v) => &mut v[index],
            value::Value::Quaternion(v) => match index {
                0 => &mut v.x,
                1 => &mut v.y,
                2 => &mut v.z,
                3 => &mut v.w,
                _ => panic!("Invalid index for Quaternion"),
            },
        }
    }
}

impl IntoIterator for value::Value {
    type Item = f32;
    type IntoIter = Box<dyn Iterator<Item = f32>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            value::Value::Float(v) => Box::new([v].into_iter()),
            value::Value::Vector2(v) => Box::new([v.x, v.y].into_iter()),
            value::Value::Vector3(v) => Box::new([v.x, v.y, v.z].into_iter()),
            value::Value::Vector4(v) => Box::new([v.x, v.y, v.z, v.w].into_iter()),
            value::Value::Quaternion(v) => Box::new([v.x, v.y, v.z, v.w].into_iter()),
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
    fn values(&self, context: &BaseProviderContext) -> value::Value {
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
) -> Vec<ValueProvider> {
    use base::BaseProviderValues;

    let mut result = Vec::new();
    let mut start = 0;

    for (i, v) in value.iter().enumerate() {
        if v.is_string() {
            close(&mut result, value.to_vec(), start, i);
            start = i + 1;
            let base = v.as_str().unwrap().to_string();
            let base_provider_values = BaseProviderValues::new(base.clone());
            result.push(ValueProvider::BaseProvider(base_provider_values));
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

    let values = value::Value::from_vec(values);

    result.push(ValueProvider::Static(r#static::StaticValues::new(values)));
}
