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
        values: Vec<Box<dyn BaseValues>>,
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
        values: Vec<Box<dyn BaseValues>>,
        _flags: Vec<String>,
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
        let point_l = points[l].get_point(context);
        let point_r = points[r].get_point(context);

        point_l.lerp(point_r, time)
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
