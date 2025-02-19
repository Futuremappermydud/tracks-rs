use crate::values::base_provider_context::BaseProviderContext;

use super::ValueProvider;

#[derive(Clone, Debug)]
pub struct StaticValues {
    pub(crate) values: Vec<f32>,
}

impl StaticValues {
    pub fn new(values: Vec<f32>) -> Self {
        Self { values }
    }
}

impl ValueProvider for StaticValues {
    fn values(&self, _context: &BaseProviderContext) -> Vec<f32> {
        self.values.clone()
    }
}
