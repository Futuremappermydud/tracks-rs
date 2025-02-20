use crate::values::base_provider_context::BaseProviderContext;
use glam::{DVec2, DVec3, DVec4};
use serde_json::Value as JsonValue;
use std::{
    any::Any,
    ops::{Add, Div, Index, IndexMut, Mul},
    sync::Arc,
};

pub mod base;
pub mod base_provider_context;
pub mod quat;
pub mod smooth;
pub mod smooth_rot;
pub mod r#static;
pub mod updatable;
pub mod value;

pub trait AbstractValueProvider {
    fn values(&self, context: &BaseProviderContext) -> value::BaseValue;
}

pub trait UpdateableValues: AbstractValueProvider {
    fn update(&mut self);
}

#[derive(Clone, Debug)]
pub enum ValueProvider {
    Static(r#static::StaticValues),
    BaseProvider(base::BaseProviderValues),
    QuaternionProvider(quat::QuaternionProviderValues),
    PartialProvider(updatable::PartialProviderValues),
    SmoothProviders(smooth::SmoothProvidersValues),
    SmoothRotationProviders(smooth_rot::SmoothRotationProvidersValues),
}

impl AbstractValueProvider for ValueProvider {
    fn values(&self, context: &BaseProviderContext) -> value::BaseValue {
        let items = match self {
            ValueProvider::Static(v) => v.values(context),
            ValueProvider::BaseProvider(v) => v.values(context),
            ValueProvider::QuaternionProvider(v) => v.values(context),
            ValueProvider::PartialProvider(v) => v.values(context),
            ValueProvider::SmoothProviders(v) => v.values(context),
            ValueProvider::SmoothRotationProviders(v) => v.values(context),
        };
        items
    }
}

// Helper function for linear interpolation
fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t.clamp(0.0, 1.0)
}

#[derive(Clone, Debug)]
pub struct JsonKeyframe {
    pub values: ValueProvider,
    pub time: f32,
}

// Values deserialization
/// Creates a new instance of [`BaseProviderValues`] using the provided base values.
///
/// # Arguments
///
/// * `base` - Clone of the base values used to initialize the provider.
#[cfg(feature = "json")]
pub fn deserialize_values(
    value: &[&JsonValue],
    _context: &BaseProviderContext,
) -> Vec<JsonKeyframe> {
    use base::BaseProviderValues;

    let mut result = Vec::new();
    let mut start = 0;

    for (i, v) in value.iter().enumerate() {
        if v.is_string() {
            close(&mut result, value.to_vec(), start, i);
            start = i + 1;
            let base = v.as_str().unwrap().to_string();

            let base_provider_values = BaseProviderValues::new(base.clone());
            let base_provider = ValueProvider::BaseProvider(base_provider_values);
            result.push(JsonKeyframe {
                values: base_provider,
                time: 0.0,
            });
        }
    }

    close(&mut result, value.to_vec(), start, value.len());
    result
}

#[cfg(feature = "json")]
fn close(result: &mut Vec<JsonKeyframe>, raw_values: Vec<&JsonValue>, open: usize, end: usize) {
    if end <= open {
        return;
    }

    let mut values: Vec<f32> = raw_values[open..end]
        .iter()
        .filter_map(|v| v.as_f64().map(|i| i as f32))
        .collect();

    let time = values.pop().unwrap_or(0.0);

    let values = value::BaseValue::from_vec(values);

    let value_provider = ValueProvider::Static(r#static::StaticValues::new(values));
    result.push(JsonKeyframe {
        values: value_provider,
        time,
    });
}
