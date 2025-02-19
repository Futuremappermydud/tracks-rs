use glam::Quat;
use glam::Vec3;

use glam::Vec2;
use glam::Vec4;

use super::lerp;

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

    #[inline(always)]
    pub fn from_vec(value: Vec<f32>) -> Value {
        Self::from_slice(value.as_slice())
    }

    pub fn from_slice(value: &[f32]) -> Value {
        match value.len() {
            1 => Value::Float(value[0]),
            2 => Value::Vector2(Vec2::new(value[0], value[1])),
            3 => Value::Vector3(Vec3::new(value[0], value[1], value[2])),
            4 => Value::Vector4(Vec4::new(value[0], value[1], value[2], value[3])),
            _ => panic!("Invalid value length"),
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            Value::Float(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_vec2(&self) -> Option<Vec2> {
        match self {
            Value::Vector2(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_vec3(&self) -> Option<Vec3> {
        match self {
            Value::Vector3(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_vec4(&self) -> Option<Vec4> {
        match self {
            Value::Vector4(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_quat(&self) -> Option<Quat> {
        match self {
            Value::Quaternion(v) => Some(*v),
            _ => None,
        }
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Float(v)
    }
}

impl From<Vec2> for Value {
    fn from(v: Vec2) -> Self {
        Value::Vector2(v)
    }
}

impl From<Vec3> for Value {
    fn from(v: Vec3) -> Self {
        Value::Vector3(v)
    }
}

impl From<Vec4> for Value {
    fn from(v: Vec4) -> Self {
        Value::Vector4(v)
    }
}

impl From<Quat> for Value {
    fn from(v: Quat) -> Self {
        Value::Quaternion(v)
    }
}
