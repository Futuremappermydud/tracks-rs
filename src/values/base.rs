use crate::values::base_provider_context::BaseProviderContext;

use super::ValueProvider;

#[derive(Clone)]
pub struct BaseProviderValues {
    pub(crate) base: String,
}

impl BaseProviderValues {
    pub fn new(base: String) -> Self {
        Self { base }
    }
}

impl ValueProvider for BaseProviderValues {
    fn values(&self, context: &BaseProviderContext) -> Vec<f32> {
        let value = context.get_values(&self.base);
        value.to_vec()
    }
}
