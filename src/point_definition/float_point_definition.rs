use glam::FloatExt;

use crate::{
    easings::functions::Functions,
    modifiers::{Modifier, float_modifier::FloatModifier, operation::Operation},
    point_data::{PointData, float_point_data::FloatPointData},
    values::{AbstractValueProvider, ValueProvider, base_provider_context::BaseProviderContext},
};

use super::PointDefinition;

pub struct FloatPointDefinition {
    points: Vec<PointData>,
}

impl PointDefinition for FloatPointDefinition {
    type Value = f32;

    fn get_count(&self) -> usize {
        self.points.len()
    }

    fn has_base_provider(&self) -> bool {
        self.points.iter().any(|p| p.has_base_provider())
    }

    fn get_points_mut(&mut self) -> &mut Vec<PointData> {
        &mut self.points
    }

    fn create_modifier(
        &self,
        values: Vec<ValueProvider>,
        modifiers: Vec<Modifier>,
        operation: Operation,
        context: &BaseProviderContext,
    ) -> Modifier {
        let mut raw_point: Option<f32> = None;
        let base_values = if values.len() == 1 {
            // Try to match the pattern: a single StaticValues with exactly one number.
            if let ValueProvider::Static(static_val) = &values[0] {
                if static_val.values(context).len() == 1 {
                    // Pattern match successful; set the value.
                    raw_point = Some(static_val.values(context)[0]);
                    None
                } else {
                    // Pattern does not match because the length is not the expected ARRAY_SIZE (1).
                    let count: usize = values.iter().map(|v| v.values(context).len()).sum();
                    assert_eq!(count, 1, "Float modifier point must have 1 number");
                    Some(values)
                }
            } else {
                // If it's a single value but not StaticValues,
                // treat it like the else branch.
                let count: usize = values.iter().map(|v| v.values(context).len()).sum();
                assert_eq!(count, 1, "Float modifier point must have 1 number");
                Some(values)
            }
        } else {
            // Multiple values provided – ensure that the total value count equals 1.
            let count: usize = values.iter().map(|v| v.values(context).len()).sum();
            assert_eq!(count, 1, "Float modifier point must have 1 number");
            Some(values)
        };
        Modifier::Float(FloatModifier::new(
            raw_point,
            base_values,
            modifiers,
            operation,
        ))
    }

    fn create_point_data(
        &self,
        values: Vec<ValueProvider>,
        _flags: Vec<String>,
        modifiers: Vec<Modifier>,
        easing: Functions,
        context: &BaseProviderContext,
    ) -> PointData {
        // If one value is present and it contains two floats, the first is the point value and the second is time.
        // [x, t]
        let mut raw_point: Option<f32> = None;
        let time: f32;

        // If there is only one value, it must be a static value.
        // If it's a timed value, extract the time and the value.
        let base_values = if values.len() == 1 {
            // [x, t]
            match &values[0] {
                ValueProvider::TimedValue(static_val, t) => {
                    raw_point = static_val.values(context).as_float();
                    time = *t;
                    None
                }
                _ => {
                    time = 0.0;
                    Some(values)
                }
            }
        } else {
            // If there are multiple values, the last one is the time.
            // [x, x, x, ..., t]
            let count: usize = values.iter().map(|v| v.values(context).len()).sum();
            if count != 2 {
                eprintln!("Float modifier point must have only 2 numbers");
            }
            time = values
                .last()
                .and_then(|v| v.values(context).last().copied())
                .unwrap_or(0.0);
            Some(values)
        };
        PointData::Float(FloatPointData::new(
            raw_point,
            base_values,
            time,
            modifiers,
            easing,
        ))
    }

    fn interpolate_points(
        &self,
        points: &[PointData],
        l: usize,
        r: usize,
        time: f32,
        context: &BaseProviderContext,
    ) -> f32 {
        let point_l = points[l].get_float(context);
        let point_r = points[r].get_float(context);

        f32::lerp(point_l, point_r, time)
    }

    fn get_points(&self) -> &Vec<PointData> {
        &self.points
    }

    fn get_point(&self, point: &PointData, context: &BaseProviderContext) -> f32 {
        point.get_float(context)
    }
}

impl FloatPointDefinition {
    /// Constructor equivalent – parses the provided JSON immediately.
    #[cfg(feature = "json")]
    pub fn new(value: &serde_json::Value, context: &BaseProviderContext) -> Self {
        let mut instance = Self { points: Vec::new() };
        instance.parse(value, context);
        instance
    }
}
