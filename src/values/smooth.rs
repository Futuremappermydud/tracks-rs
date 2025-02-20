use serde_json::de;

use super::lerp;

use super::UpdateableValues;
use super::value::BaseValue;

use crate::values::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;

pub struct SmoothProvidersValues {
    pub(crate) source: BaseValue,
    pub(crate) mult: f32,
    pub(crate) values: BaseValue,
}

impl SmoothProvidersValues {
    pub fn new(source: BaseValue, mult: f32) -> Self {
        Self {
            source: source.clone(),
            mult,
            values: source * 0.0,
        }
    }
}

impl AbstractValueProvider for SmoothProvidersValues {
    fn values(&self, _context: &BaseProviderContext) -> BaseValue {
        self.values.clone()
    }
}

impl UpdateableValues for SmoothProvidersValues {
    fn update(&mut self) {
        // Note: You'll need to implement your own time delta functionality
        let delta = 0.016666667 * self.mult; // Example: 60 FPS

        for i in 0..self.source.len() {
            self.values[i] = lerp(self.values[i], self.source[i], delta);
        }
    }
}
