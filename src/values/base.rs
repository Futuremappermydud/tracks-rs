use crate::values::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;

#[derive(Clone)]
pub struct BaseProviderValues {
    pub(crate) base: String,
}

impl BaseProviderValues {
    pub fn new(base: String) -> Self {
        Self { base }
    }
}

impl AbstractValueProvider for BaseProviderValues {
    fn values(&self, context: &BaseProviderContext) -> Vec<f32> {
        let base = self.base.split(".").collect::<Vec<&str>>();
        let value = context.get_values(&base[0]);
        value
    }
}
