use std::borrow::Cow;

use crate::values::base_provider_context::BaseProviderContext;

use super::{AbstractValueProvider, value::BaseValue};

#[derive(Clone, Debug)]
pub struct BaseProviderValues {
    pub(crate) base: String,
}

impl BaseProviderValues {
    pub fn new(base: String) -> Self {
        Self { base }
    }
}

impl AbstractValueProvider for BaseProviderValues {
    fn values<'a>(&'a self, context: &BaseProviderContext) -> Cow<'a, [f32]> {
        let base = self.base.split(".").collect::<Vec<&str>>();
        let value = context.get_values(&base[0]);
        value.as_slice().to_vec().into()
    }
}
