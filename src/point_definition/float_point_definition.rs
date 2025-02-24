use glam::FloatExt;

use crate::{
    easings::functions::Functions,
    modifiers::{
        Modifier,
        float_modifier::{FloatModifier, FloatValues},
        operation::Operation,
    },
    point_data::{PointData, float_point_data::FloatPointData},
    values::{
        AbstractValueProvider, ValueProvider,
        base_provider_context::BaseProviderContext,
    },
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
        let val = match values.as_slice() {
            // Single static value
            [ValueProvider::Static(static_val)] if static_val.values(context).len() == 1 => {
                FloatValues::Static(static_val.values(context)[0])
            }
            // Any other case
            _ => {
                let count: usize = values.iter().map(|v| v.values(context).len()).sum();
                assert_eq!(count, 1, "Float modifier point must have 1 number");
                FloatValues::Dynamic(values)
            }
        };
        Modifier::Float(FloatModifier::new(val, modifiers, operation))
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

        let (value, time) = match &values[..] {
            // [x, y]
            [ValueProvider::Static(static_val)] => {
                let mut static_val = static_val.values.clone();

                let time = static_val.pop().unwrap_or_default();
                let val = static_val.get(0).copied();

                let point_val = FloatValues::Static(val.unwrap_or_default());
                (point_val, time)
            }

            _ => {
                // validate and get time
                let raw_values = values.iter().map(|v| v.values(context)).collect::<Vec<_>>();

                let time = raw_values
                    .last()
                    .and_then(|v| v.last().copied())
                    .unwrap_or(0.0);

                let count: usize = raw_values.iter().map(|v| v.len()).sum();
                if count != 2 {
                    eprintln!("Float modifier point must have 2 numbers");
                }

                let value = FloatValues::Dynamic(values);

                (value, time)
            }
        };

        PointData::Float(FloatPointData::new(value, time, modifiers, easing))
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
    pub fn new(value: serde_json::Value, context: &BaseProviderContext) -> Self {
        let mut instance = Self { points: Vec::new() };
        instance.parse(value, context);
        instance
    }
}
