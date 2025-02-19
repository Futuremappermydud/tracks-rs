use super::{UpdateableValues, Value};

use crate::values::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;

pub struct PartialProviderValues {
    pub(crate) source: Value,
    pub(crate) parts: Vec<usize>,
    pub(crate) values: Value,
}

impl PartialProviderValues {
    pub fn new(source: Value, parts: Vec<usize>) -> Self {
        Self {
            source,
            values: Value::from_vec(vec![0.0; parts.len()]),
            parts,
        }
    }
}

impl AbstractValueProvider for PartialProviderValues {
    fn values(&self, _context: &BaseProviderContext) -> Value {
        self.values.clone()
    }
}

impl UpdateableValues for PartialProviderValues {
    fn update(&mut self) {
        for (i, &part) in self.parts.iter().enumerate() {
            self.values[i] = self.source[part];
        }
    }
}
