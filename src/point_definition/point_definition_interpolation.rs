use crate::values::{base_provider_context::BaseProviderContext, value::BaseValue};

use super::{BasePointDefinition, PointDefinition};

use glam::FloatExt;

struct PointDefinitionInterpolation {
    base_point_data: Option<BasePointDefinition>,
    previous_point_data: Option<BasePointDefinition>,
    time: f32,
}

impl PointDefinitionInterpolation {
    pub fn finish(&mut self) {
        self.previous_point_data = None;
    }

    pub fn init(&mut self, new_point_data: Option<BasePointDefinition>) {
        self.time = 0.0;
        self.previous_point_data = self.base_point_data.take();

        match new_point_data {
            None => {
                self.base_point_data = None;
            }
            Some(point_data) => {
                self.base_point_data = Some(point_data);
            }
        }
    }

    pub fn interpolate(&self, time: f32, context: &BaseProviderContext) -> Option<BaseValue> {
        let base_data = self.base_point_data.as_ref()?;

        let v = match &self.previous_point_data {
            None => base_data.interpolate(time, context).0,
            Some(prev_data) => {
                Self::interpolate_points(prev_data, base_data, self.time, time, context)
            }
        };

        Some(v)
    }

    fn interpolate_points(
        previous_point: &BasePointDefinition,
        base_point: &BasePointDefinition,
        interpolation: f32,
        time: f32,
        context: &BaseProviderContext,
    ) -> BaseValue {
        

        BaseValue::lerp(
            previous_point.interpolate(time, context).0,
            base_point.interpolate(time, context).0,
            interpolation,
        )
    }
}

// impl<T> std::fmt::Display for PointDefinitionInterpolation<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "({}, {:?})",
//             self.previous_point_data
//                 .as_ref()
//                 .map_or("null".to_string(), |p| p.to_string()),
//             self.base_point_data
//         )
//     }
// }
