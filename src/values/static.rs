use crate::values::base_provider_context::BaseProviderContext;

use super::{AbstractValueProvider, value::BaseValue};

#[derive(Clone, Debug)]
pub struct StaticValues {
    pub(crate) values: BaseValue,
}

impl StaticValues {
    pub fn new(values: BaseValue) -> Self {
        Self { values }
    }
}

impl AbstractValueProvider for StaticValues {
    fn values(&self, _context: &BaseProviderContext) -> BaseValue {
        self.values.clone()
    }
}
