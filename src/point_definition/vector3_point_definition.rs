use glam::Vec3;

use crate::{
    easings::functions::Functions,
    modifiers::{ModifierBase, operation::Operation, vector3_modifier::Vector3Modifier},
    point_data::{BasePointData, vector3_point_data::Vector3PointData},
    values::base_provider_context::BaseProviderContext,
    values::{BaseValues, StaticValues, Values},
};

use super::PointDefinition;

pub struct Vector3PointDefinition {
    points: Vec<Box<dyn BasePointData<Vec3>>>,
}

impl Vector3PointDefinition {
    fn smooth_vector_lerp(
        &self,
        points: &[Box<dyn BasePointData<Vec3>>],
        l: usize,
        r: usize,
        time: f32,
        context: &BaseProviderContext,
    ) -> Vec3 {
        let point_a = points[l].get_point(context);
        let point_b = points[r].get_point(context);

        // Catmull-Rom Spline
        let p0 = if l > 0 {
            points[l - 1].get_point(context)
        } else {
            point_a
        };
        let p3 = if r + 1 < points.len() {
            points[r + 1].get_point(context)
        } else {
            point_b
        };

        let tt = time * time;
        let ttt = tt * time;

        let q0 = -ttt + (2.0 * tt) - time;
        let q1 = (3.0 * ttt) - (5.0 * tt) + 2.0;
        let q2 = (-3.0 * ttt) + (4.0 * tt) + time;
        let q3 = ttt - tt;

        0.5 * ((p0 * q0) + (point_a * q1) + (point_b * q2) + (p3 * q3))
    }
}

impl PointDefinition for Vector3PointDefinition {
    type Value = Vec3;

    fn get_count(&self) -> usize {
        self.points.len()
    }

    fn has_base_provider(&self) -> bool {
        self.points.iter().any(|p| p.has_base_provider())
    }

    fn get_points_mut(&mut self) -> &mut Vec<Box<dyn BasePointData<Self::Value>>> {
        &mut self.points
    }

    fn create_modifier(
        &self,
        values: Vec<ValueProvider>,
        modifiers: Vec<Box<dyn ModifierBase<Value = Vec3>>>,
        operation: Operation,
        context: &BaseProviderContext,
    ) -> Box<dyn ModifierBase<Value = Vec3>> {
        let mut raw_point: Option<Vec3> = None;
        let base_values = if values.len() == 1 {
            if let Some(static_val) = values[0].as_ref().as_any().downcast_ref::<StaticValues>() {
                if static_val.values(context).len() == 3 {
                    raw_point = Some(Vec3::new(
                        static_val.values(context)[0],
                        static_val.values(context)[1],
                        static_val.values(context)[2],
                    ));
                    None
                } else {
                    let count: usize = values.iter().map(|v| v.values(context).len()).sum();
                    assert_eq!(count, 3, "Vector3 modifier point must have 3 numbers");
                    Some(values)
                }
            } else {
                let count: usize = values.iter().map(|v| v.values(context).len()).sum();
                assert_eq!(count, 3, "Vector3 modifier point must have 3 numbers");
                Some(values)
            }
        } else {
            let count: usize = values.iter().map(|v| v.values(context).len()).sum();
            assert_eq!(count, 3, "Vector3 modifier point must have 3 numbers");
            Some(values)
        };
        Box::new(Vector3Modifier::new(
            raw_point,
            base_values,
            modifiers,
            operation,
        ))
    }

    fn create_point_data(
        &self,
        values: Vec<ValueProvider>,
        flags: Vec<String>,
        modifiers: Vec<Box<dyn ModifierBase<Value = Vec3>>>,
        easing: Functions,
        context: &BaseProviderContext,
    ) -> Box<dyn BasePointData<Vec3>> {
        let mut raw_point: Option<Vec3> = None;
        let time: f32;
        let base_values = if values.len() == 1 {
            if let Some(static_val) = values[0].as_ref().as_any().downcast_ref::<StaticValues>() {
                if static_val.values(context).len() == 4 {
                    raw_point = Some(Vec3::new(
                        static_val.values(context)[0],
                        static_val.values(context)[1],
                        static_val.values(context)[2],
                    ));
                    time = static_val.values(context)[3];
                    None
                } else {
                    time = 0.0;
                    Some(values)
                }
            } else {
                time = 0.0;
                Some(values)
            }
        } else {
            let count: usize = values.iter().map(|v| v.values(context).len()).sum();
            if count != 4 {
                eprintln!("Vector3 point must have 4 numbers");
            }
            time = values
                .last()
                .and_then(|v| v.values(context).last().copied())
                .unwrap_or(0.0);
            Some(values)
        };
        Box::new(Vector3PointData::new(
            raw_point,
            base_values,
            flags.iter().any(|f| f == "splineCatmullRom"),
            time,
            modifiers,
            easing,
        ))
    }

    fn interpolate_points(
        &self,
        points: &[Box<dyn BasePointData<Vec3>>],
        l: usize,
        r: usize,
        time: f32,
        context: &BaseProviderContext,
    ) -> Vec3 {
        let point_r = points[r]
            .as_any()
            .downcast_ref::<Vector3PointData>()
            .unwrap();

        if point_r.smooth {
            self.smooth_vector_lerp(points, l, r, time, context)
        } else {
            let point_l = points[l].get_point(context);
            let point_r = points[r].get_point(context);
            point_l.lerp(point_r, time)
        }
    }

    fn get_points(&self) -> &Vec<Box<dyn BasePointData<Vec3>>> {
        &self.points
    }
}

impl Vector3PointDefinition {
    #[cfg(feature = "json")]
    pub fn new(value: &serde_json::Value, context: &BaseProviderContext) -> Self {
        let mut instance = Self { points: Vec::new() };
        instance.parse(value, context);
        instance
    }
}
