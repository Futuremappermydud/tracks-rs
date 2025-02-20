use super::{UpdateableValues, value::BaseValue};

use crate::values::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;


#[derive(Clone, Debug)]
pub struct PartialProviderValues {
    pub(crate) source: BaseValue,
    pub(crate) parts: Vec<usize>,
    pub(crate) values: BaseValue,
}

impl PartialProviderValues {
    pub fn new(source: BaseValue, parts: Vec<usize>) -> Self {
        Self {
            source,
            values: BaseValue::from_vec(vec![0.0; parts.len()]),
            parts,
        }
    }
}

impl AbstractValueProvider for PartialProviderValues {
    fn values(&self, _context: &BaseProviderContext) -> BaseValue {
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
