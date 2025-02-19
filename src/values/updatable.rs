use super::UpdateableValues;

use crate::values::base_provider_context::BaseProviderContext;

use super::ValueProvider;

pub struct PartialProviderValues {
    pub(crate) source: Vec<f32>,
    pub(crate) parts: Vec<usize>,
    pub(crate) values: Vec<f32>,
}

impl PartialProviderValues {
    pub fn new(source: Vec<f32>, parts: Vec<usize>) -> Self {
        Self {
            source,
            values: vec![0.0; parts.len()],
            parts,
        }
    }
}

impl ValueProvider for PartialProviderValues {
    fn values(&self, _context: &BaseProviderContext) -> Vec<f32> {
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
