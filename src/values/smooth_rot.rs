use super::UpdateableValues;

use crate::values::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;

use glam::Quat;

pub struct SmoothRotationProvidersValues {
    pub(crate) rotation_values: Quat,
    pub(crate) mult: f32,
    pub(crate) last_quaternion: Quat,
    pub(crate) values: Vec<f32>,
}

impl SmoothRotationProvidersValues {
    pub fn new(rotation_values: Quat, mult: f32) -> Self {
        Self {
            rotation_values,
            mult,
            last_quaternion: Quat::IDENTITY,
            values: vec![0.0; 3],
        }
    }
}

impl AbstractValueProvider for SmoothRotationProvidersValues {
    fn values(&self, _context: &BaseProviderContext) -> Vec<f32> {
        self.values.clone()
    }
}

impl UpdateableValues for SmoothRotationProvidersValues {
    fn update(&mut self) {
        // Note: You'll need to implement your own time delta functionality
        let delta_time = 0.016666667; // Example: 60 FPS
        self.last_quaternion = self
            .last_quaternion
            .slerp(self.rotation_values.rotation(), delta_time * self.mult);
        let euler = self.last_quaternion.to_euler(glam::EulerRot::XYZ);
        self.values[0] = euler.0.to_degrees();
        self.values[1] = euler.1.to_degrees();
        self.values[2] = euler.2.to_degrees();
    }
}
