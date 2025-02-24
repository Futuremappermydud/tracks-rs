use std::borrow::Cow;

use super::UpdateableValues;

use crate::{
    modifiers::quaternion_modifier::TRACKS_EULER_ROT,
    values::base_provider_context::BaseProviderContext,
};

use super::AbstractValueProvider;

use glam::Quat;

#[derive(Clone, Debug)]
pub struct QuaternionProviderValues {
    pub(crate) source: Vec<f32>,
    pub(crate) values: [f32; 3],
    pub(crate) rotation: Quat,
}

impl QuaternionProviderValues {
    pub fn new(source: Vec<f32>) -> Self {
        Self {
            source,
            values: Default::default(),
            rotation: Quat::IDENTITY,
        }
    }
}

impl AbstractValueProvider for QuaternionProviderValues {
    fn values<'a>(&'a self, _context: &BaseProviderContext) -> Cow<'a, [f32]> {
        Cow::Borrowed(self.values.as_ref())
    }
}

impl UpdateableValues for QuaternionProviderValues {
    fn update(&mut self, _delta: f32) {
        self.rotation = Quat::from_xyzw(
            self.source[0],
            self.source[1],
            self.source[2],
            self.source[3],
        );
        let euler = self.rotation.to_euler(TRACKS_EULER_ROT);

        self.values = [
            euler.0.to_degrees(),
            euler.1.to_degrees(),
            euler.2.to_degrees(),
        ];
    }
}
