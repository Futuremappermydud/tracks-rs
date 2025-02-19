use crate::values::base_provider_context::BaseProviderContext;

use super::{AbstractValueProvider, Value};

#[derive(Clone, Debug)]
pub struct StaticValues {
    pub(crate) values: Value,
}

impl StaticValues {
    pub fn new(values: Value) -> Self {
        Self { values }
    }
}

impl AbstractValueProvider for StaticValues {
    fn values(&self, _context: &BaseProviderContext) -> Value {
        self.values.clone()
    }
}
