use std::borrow::Cow;

use super::{ValueProvider, value::BaseValue};

use crate::{
    modifiers::quaternion_modifier::TRACKS_EULER_ROT,
    values::base_provider_context::BaseProviderContext,
};

use super::AbstractValueProvider;

use glam::Quat;

#[derive(Clone, Debug)]
pub struct QuaternionProviderValues {
    pub(crate) source: Box<ValueProvider>,
}

impl QuaternionProviderValues {
    pub fn new(source: ValueProvider) -> Self {
        Self {
            source: Box::new(source),
        }
    }
}

impl AbstractValueProvider for QuaternionProviderValues {
    fn values<'a>(&'a self, _context: &BaseProviderContext) -> Cow<'a, [f32]> {
        let source = self.source.values(_context);
        let rotation = Quat::from_xyzw(source[0], source[1], source[2], source[3]);
        let euler = rotation.to_euler(TRACKS_EULER_ROT);

        let values = [
            euler.0.to_degrees(),
            euler.1.to_degrees(),
            euler.2.to_degrees(),
        ];
        values.to_vec().into()
    }
}
