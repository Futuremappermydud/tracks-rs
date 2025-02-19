use crate::values::base_provider_context::BaseProviderContext;
use glam::{DVec2, DVec3, DVec4, Quat, Vec2, Vec3, Vec4};
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

pub trait AbstractValueProvider {
    fn values(&self, context: &BaseProviderContext) -> Value;

    fn as_float(&self, context: &BaseProviderContext) -> Option<f32> {
        match self.values(context) {
            Value::Float(v) => Some(v),
            _ => None,
        }
    }

    fn as_vector2(&self, context: &BaseProviderContext) -> Option<Vec2> {
        match self.values(context) {
            Value::Vector2(v) => Some(v),
            _ => None,
        }
    }

    fn as_vector3(&self, context: &BaseProviderContext) -> Option<Vec3> {
        match self.values(context) {
            Value::Vector3(v) => Some(v),
            _ => None,
        }
    }

    fn as_vector4(&self, context: &BaseProviderContext) -> Option<Vec4> {
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

#[derive(Clone, Debug, Copy)]
pub enum Value {
    Float(f32),
    Vector2(Vec2),
    Vector3(Vec3),
    Vector4(Vec4),
    Quaternion(Quat),
}

impl Value {
    pub fn lerp(&self, end: &Value, t: f32) -> Value {
        match (self, end) {
            (Value::Float(start), Value::Float(end)) => Value::Float(lerp(*start, *end, t)),
            (Value::Vector2(start), Value::Vector2(end)) => {
                Value::Vector2(Vec2::lerp(*start, *end, t))
            }
            (Value::Vector3(start), Value::Vector3(end)) => {
                Value::Vector3(Vec3::lerp(*start, *end, t))
            }
            (Value::Vector4(start), Value::Vector4(end)) => {
                Value::Vector4(Vec4::lerp(*start, *end, t))
            }
            (Value::Quaternion(start), Value::Quaternion(end)) => {
                Value::Quaternion(Quat::slerp(*start, *end, t))
            }
            _ => panic!("Invalid value types"),
        }
    }

    pub fn from_vec(value: Vec<f32>) -> Value {
        match value.len() {
            1 => Value::Float(value[0]),
            2 => Value::Vector2(Vec2::new(value[0], value[1])),
            3 => Value::Vector3(Vec3::new(value[0], value[1], value[2])),
            4 => Value::Vector4(Vec4::new(value[0], value[1], value[2], value[3])),
            _ => panic!("Invalid value length"),
        }
    }
}

// scalar ops

impl Mul<f32> for Value {
    type Output = Value;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Value::Float(v) => Value::Float(v * rhs),
            Value::Vector2(v) => Value::Vector2(v * rhs),
            Value::Vector3(v) => Value::Vector3(v * rhs),
            Value::Vector4(v) => Value::Vector4(v * rhs),
            Value::Quaternion(v) => Value::Quaternion(v * rhs),
        }
    }
}

impl Div<f32> for Value {
    type Output = Value;

    fn div(self, rhs: f32) -> Self::Output {
        match self {
            Value::Float(v) => Value::Float(v / rhs),
            Value::Vector2(v) => Value::Vector2(v / rhs),
            Value::Vector3(v) => Value::Vector3(v / rhs),
            Value::Vector4(v) => Value::Vector4(v / rhs),
            Value::Quaternion(v) => Value::Quaternion(v / rhs),
        }
    }
}

impl Index<usize> for Value {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Value::Float(f) => f,
            Value::Vector2(v) => &v[index],
            Value::Vector3(v) => &v[index],
            Value::Vector4(v) => &v[index],
            Value::Quaternion(v) => match index {
                0 => &v.x,
                1 => &v.y,
                2 => &v.z,
                3 => &v.w,
                _ => panic!("Invalid index for Quaternion"),
            },
        }
    }
}
impl IndexMut<usize> for Value {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Value::Float(f) => f,
            Value::Vector2(v) => &mut v[index],
            Value::Vector3(v) => &mut v[index],
            Value::Vector4(v) => &mut v[index],
            Value::Quaternion(v) => match index {
                0 => &mut v.x,
                1 => &mut v.y,
                2 => &mut v.z,
                3 => &mut v.w,
                _ => panic!("Invalid index for Quaternion"),
            },
        }
    }
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

    let values = Value::from_vec(values);

    result.push(ValueProvider::Static(r#static::StaticValues::new(values)));
}
