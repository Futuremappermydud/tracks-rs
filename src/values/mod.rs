use glam::Quat;
use serde_json::Value as JsonValue;
use std::{any::Any, sync::Arc, cell::RefCell};
use crate::values::base_provider_context::BaseProviderContext;

pub mod base_provider_context;

pub trait Values {
    fn values(&self, context: &BaseProviderContext) -> Vec<f32>;
}

pub trait RotationValues {
    fn rotation(&self) -> Quat;
}

pub trait BaseValues: Values {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, Debug)]
pub struct StaticValues {
    values: Vec<f32>,
}

impl StaticValues {
    pub fn new(values: Vec<f32>) -> Self {
        Self { values }
    }
}

impl Values for StaticValues {
    fn values(&self, _context: &BaseProviderContext) -> Vec<f32> {
        self.values.clone()
    }
}

impl BaseValues for StaticValues {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct BaseProviderValues {
    base: String
}

impl BaseProviderValues {
    pub fn new(base: String) -> Self {
        Self { base }
    }
}

impl Values for BaseProviderValues {
    fn values(&self, context: &BaseProviderContext) -> Vec<f32> {
        let value = context.base_combo.borrow();
        value.to_vec()
    }
}

impl BaseValues for BaseProviderValues {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait UpdateableValues: Values {
    fn update(&mut self);
}

pub struct QuaternionProviderValues {
    source: Vec<f32>,
    values: Vec<f32>,
    rotation: Quat,
}

impl QuaternionProviderValues {
    pub fn new(source: Vec<f32>) -> Self {
        Self {
            source,
            values: vec![0.0; 3],
            rotation: Quat::IDENTITY,
        }
    }
}

impl Values for QuaternionProviderValues {
    fn values(&self, _context: &BaseProviderContext) -> Vec<f32> {
        self.values.clone()
    }
}

impl RotationValues for QuaternionProviderValues {
    fn rotation(&self) -> Quat {
        self.rotation
    }
}

impl UpdateableValues for QuaternionProviderValues {
    fn update(&mut self) {
        self.rotation = Quat::from_xyzw(
            self.source[0],
            self.source[1],
            self.source[2],
            self.source[3],
        );
        let euler = self.rotation.to_euler(glam::EulerRot::XYZ);
        self.values[0] = euler.0.to_degrees();
        self.values[1] = euler.1.to_degrees();
        self.values[2] = euler.2.to_degrees();
    }
}

pub struct PartialProviderValues<'a> {
    source: &'a [f32],
    parts: Vec<usize>,
    values: Vec<f32>,
}

impl<'a> PartialProviderValues<'a> {
    pub fn new(source: &'a [f32], parts: Vec<usize>) -> Self {
        Self {
            source,
            values: vec![0.0; parts.len()],
            parts,
        }
    }
}

impl<'a> Values for PartialProviderValues<'a> {
    fn values(&self, _context: &BaseProviderContext) -> Vec<f32> {
        self.values.clone()
    }
}

impl<'a> UpdateableValues for PartialProviderValues<'a> {
    fn update(&mut self) {
        for (i, &part) in self.parts.iter().enumerate() {
            self.values[i] = self.source[part];
        }
    }
}

pub struct SmoothRotationProvidersValues {
    rotation_values: Arc<dyn RotationValues>,
    mult: f32,
    last_quaternion: Quat,
    values: Vec<f32>,
}

impl SmoothRotationProvidersValues {
    pub fn new(rotation_values: Arc<dyn RotationValues>, mult: f32) -> Self {
        Self {
            rotation_values,
            mult,
            last_quaternion: Quat::IDENTITY,
            values: vec![0.0; 3],
        }
    }
}

impl Values for SmoothRotationProvidersValues {
    fn values(&self, _context: &BaseProviderContext) -> Vec<f32> {
        self.values.clone()
    }
}

impl UpdateableValues for SmoothRotationProvidersValues {
    fn update(&mut self) {
        // Note: You'll need to implement your own time delta functionality
        let delta_time = 0.016666667; // Example: 60 FPS
        self.last_quaternion = self
            .last_quaternion
            .slerp(self.rotation_values.rotation(), delta_time * self.mult);
        let euler = self.last_quaternion.to_euler(glam::EulerRot::XYZ);
        self.values[0] = euler.0.to_degrees();
        self.values[1] = euler.1.to_degrees();
        self.values[2] = euler.2.to_degrees();
    }
}

pub struct SmoothProvidersValues {
    source: Vec<f32>,
    mult: f32,
    values: Vec<f32>,
}

impl SmoothProvidersValues {
    pub fn new(source: Vec<f32>, mult: f32) -> Self {
        Self {
            source: source.clone(),
            mult,
            values: vec![0.0; source.len()],
        }
    }
}

impl Values for SmoothProvidersValues {
    fn values(&self, _context: &BaseProviderContext) -> Vec<f32> {
        self.values.clone()
    }
}

impl UpdateableValues for SmoothProvidersValues {
    fn update(&mut self) {
        // Note: You'll need to implement your own time delta functionality
        let delta = 0.016666667 * self.mult; // Example: 60 FPS
        for i in 0..self.source.len() {
            self.values[i] = lerp(self.values[i], self.source[i], delta);
        }
    }
}

// Helper function for linear interpolation
fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t.clamp(0.0, 1.0)
}

// Values deserialization
#[cfg(feature = "json")]
pub fn deserialize_values(value: &[&JsonValue], _context: &BaseProviderContext) -> Vec<Box<dyn BaseValues>> {
    let mut result = Vec::new();
    let mut start = 0;

    for (i, v) in value.iter().enumerate() {
      if v.is_string() {
        close(&mut result, value.to_vec(), start, i);
        start = i + 1;
        let base = v.as_str().unwrap().to_string();
        result.push(Box::new(BaseProviderValues::new(base.clone())));
      }
    }

    close(&mut result, value.to_vec(), start, value.len());
    result
}

#[cfg(feature = "json")]
fn close(result: &mut Vec<Box<dyn BaseValues>>, raw_values: Vec<&JsonValue>, open: usize, end: usize) {
    if end <= open {
        return;
    }

    let values: Vec<f32> = raw_values[open..end]
        .iter()
        .filter_map(|v| v.as_f64().map(|i| i as f32))
        .collect();

    result.push(Box::new(StaticValues::new(values)));
}
