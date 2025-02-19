use super::{UpdateableValues, value::Value};

use crate::values::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;

use glam::{Quat, vec3};

pub struct SmoothRotationProvidersValues {
    pub(crate) rotation_values: Quat,
    pub(crate) mult: f32,
    pub(crate) last_quaternion: Quat,
    pub(crate) values: Value,
}

impl SmoothRotationProvidersValues {
    pub fn new(rotation_values: Quat, mult: f32) -> Self {
        Self {
            rotation_values,
            mult,
            last_quaternion: Quat::IDENTITY,
            values: Value::Vector3(Default::default()),
        }
    }
}

impl AbstractValueProvider for SmoothRotationProvidersValues {
    fn values(&self, _context: &BaseProviderContext) -> Value {
        self.values
    }
}

impl UpdateableValues for SmoothRotationProvidersValues {
    fn update(&mut self) {
        // Note: You'll need to implement your own time delta functionality
        let delta_time = 0.016666667; // Example: 60 FPS
        self.last_quaternion = self
            .last_quaternion
            .slerp(self.rotation_values, delta_time * self.mult);
        let euler = self.last_quaternion.to_euler(glam::EulerRot::XYZ);
        let vec = Value::Vector3(vec3(
            euler.0.to_degrees(),
            euler.1.to_degrees(),
            euler.2.to_degrees(),
        ));
        self.values = vec;
    }
}
