use serde_json::de;

use super::lerp;

use super::UpdateableValues;
use super::Value;

use crate::values::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;

pub struct SmoothProvidersValues {
    pub(crate) source: Value,
    pub(crate) mult: f32,
    pub(crate) values: Value,
}

impl SmoothProvidersValues {
    pub fn new(source: Value, mult: f32) -> Self {
        Self {
            source: source.clone(),
            mult,
            values: source * 0.0,
        }
    }
}

impl AbstractValueProvider for SmoothProvidersValues {
    fn values(&self, _context: &BaseProviderContext) -> Value {
        self.values.clone()
    }
}

impl UpdateableValues for SmoothProvidersValues {
    fn update(&mut self) {
        // Note: You'll need to implement your own time delta functionality
        let delta = 0.016666667 * self.mult; // Example: 60 FPS

        self.values = self.source.lerp(&self.values, delta);
    }
}
