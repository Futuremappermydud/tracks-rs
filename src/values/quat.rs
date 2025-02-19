use super::{UpdateableValues, Value};

use crate::values::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;

use glam::{Quat, vec3};

pub struct QuaternionProviderValues {
    pub(crate) source: Vec<f32>,
    pub(crate) values: Value,
    pub(crate) rotation: Quat,
}

impl QuaternionProviderValues {
    pub fn new(source: Vec<f32>) -> Self {
        Self {
            source,
            values: Value::Vector3(Default::default()),
            rotation: Quat::IDENTITY,
        }
    }
}

impl AbstractValueProvider for QuaternionProviderValues {
    fn values(&self, _context: &BaseProviderContext) -> Value {
        self.values.clone()
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
        let euler = self.rotation.to_euler(glam::EulerRot::ZXY);
        let vec = Value::Vector3(vec3(
            euler.0.to_degrees(),
            euler.1.to_degrees(),
            euler.2.to_degrees(),
        ));
        self.values = vec;
    }
}
