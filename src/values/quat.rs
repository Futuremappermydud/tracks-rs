use super::UpdateableValues;

use super::RotationValues;

use crate::values::base_provider_context::BaseProviderContext;

use super::ValueProvider;

use glam::Quat;

pub struct QuaternionProviderValues {
    pub(crate) source: Vec<f32>,
    pub(crate) values: Vec<f32>,
    pub(crate) rotation: Quat,
}

impl QuaternionProviderValues {
    pub fn new(source: Vec<f32>) -> Self {
        Self {
            source,
            values: vec![0.0; 3],
            rotation: Quat::IDENTITY,
        }
    }
}

impl ValueProvider for QuaternionProviderValues {
    fn values(&self, _context: &BaseProviderContext) -> Vec<f32> {
        self.values.clone()
    }
}

impl RotationValues for QuaternionProviderValues {
    fn rotation(&self) -> Quat {
        self.rotation
    }
}

impl UpdateableValues for QuaternionProviderValues {
    fn update(&mut self) {
        self.rotation = Quat::from_xyzw(
            self.source[0],
            self.source[1],
            self.source[2],
            self.source[3],
        );
        let euler = self.rotation.to_euler(glam::EulerRot::XYZ);
        self.values[0] = euler.0.to_degrees();
        self.values[1] = euler.1.to_degrees();
        self.values[2] = euler.2.to_degrees();
    }
}
