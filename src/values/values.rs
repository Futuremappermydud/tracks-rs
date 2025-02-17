use glam::Quat;
use serde_json::Value;
use std::{any::Any, sync::Arc};

pub trait Values {
    fn values(&self) -> &[f32];
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
    fn values(&self) -> &[f32] {
        &self.values
    }
}

impl BaseValues for StaticValues {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct BaseProviderValues {
    values: Vec<f32>,
}

impl BaseProviderValues {
    pub fn new(values: Vec<f32>) -> Self {
        Self { values }
    }
}

impl Values for BaseProviderValues {
    fn values(&self) -> &[f32] {
        &self.values
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
    fn values(&self) -> &[f32] {
        &self.values
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

pub struct PartialProviderValues {
    source: Vec<f32>,
    parts: Vec<usize>,
    values: Vec<f32>,
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

impl Values for PartialProviderValues {
    fn values(&self) -> &[f32] {
        &self.values
    }
}

impl UpdateableValues for PartialProviderValues {
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
    fn values(&self) -> &[f32] {
        &self.values
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
    fn values(&self) -> &[f32] {
        &self.values
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
pub fn deserialize_values(value: &[&Value]) -> Vec<Box<dyn BaseValues>> {
    let mut result = Vec::new();
    let start = 0;

    close(&mut result, value.to_vec(), start, value.len());
    result
}

fn close(result: &mut Vec<Box<dyn BaseValues>>, raw_values: Vec<&Value>, open: usize, end: usize) {
    if end <= open {
        return;
    }

    let values: Vec<f32> = raw_values[open..end]
        .iter()
        .filter_map(|v| v.as_f64().map(|i| i as f32))
        .collect();

    result.push(Box::new(StaticValues::new(values)));
}
