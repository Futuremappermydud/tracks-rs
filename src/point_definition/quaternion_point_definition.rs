use glam::{EulerRot, Quat, Vec3};
use palette::angle::RealAngle;

use crate::{
    easings::functions::Functions,
    modifiers::{
        ModifierBase, operation::Operation, quaternion_modifier::QuaternionModifier,
        vector3_modifier::Vector3Modifier,
    },
    point_data::{
        BasePointData, quaternion_point_data::QuaternionPointData,
        vector3_point_data::Vector3PointData,
    },
    values::{BaseValues, StaticValues, Values, base_provider_context::BaseProviderContext},
};

use super::PointDefinition;

pub struct QuaternionPointDefinition {
    points: Vec<Box<dyn BasePointData<Quat>>>,
}

impl PointDefinition for QuaternionPointDefinition {
    type Value = Quat;

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
        modifiers: Vec<Box<dyn ModifierBase<Value = Quat>>>,
        operation: Operation,
        context: &BaseProviderContext,
    ) -> Box<dyn ModifierBase<Value = Quat>> {
        let mut raw_vector_point: Option<Vec3> = None;
        let base_values = if values.len() == 1 {
            if let Some(static_val) = values[0].as_ref().as_any().downcast_ref::<StaticValues>() {
                if static_val.values(context).len() == 3 {
                    raw_vector_point = Some(Vec3::new(
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
        Box::new(QuaternionModifier::new(
            if raw_vector_point.is_none() {
                None
            } else {
                Some(Quat::from_euler(
                    EulerRot::XYZ,
                    raw_vector_point.unwrap().x.to_radians(),
                    raw_vector_point.unwrap().y.to_radians(),
                    raw_vector_point.unwrap().z.to_radians(),
                ))
            },
            raw_vector_point,
            base_values,
            modifiers,
            operation,
        ))
    }

    fn create_point_data(
        &self,
        values: Vec<Box<dyn BaseValues>>,
        flags: Vec<String>,
        modifiers: Vec<Box<dyn ModifierBase<Value = Quat>>>,
        easing: Functions,
        context: &BaseProviderContext,
    ) -> Box<dyn BasePointData<Quat>> {
        let mut raw_vector_point: Option<Vec3> = None;
        let time: f32;
        let base_values = if values.len() == 1 {
            if let Some(static_val) = values[0].as_ref().as_any().downcast_ref::<StaticValues>() {
                if static_val.values(context).len() == 4 {
                    raw_vector_point = Some(Vec3::new(
                        static_val.values(context)[0],
                        static_val.values(context)[1],
                        static_val.values(context)[2],
                    ));
                    println!("raw_vector_point: {:?}", raw_vector_point);
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
        Box::new(QuaternionPointData::new(
            if raw_vector_point.is_none() {
                None
            } else {
                println!("raw_vector_point: {:?}", raw_vector_point);
                Some(Quat::from_euler(
                    EulerRot::XYZ,
                    raw_vector_point.unwrap().x.to_radians(),
                    raw_vector_point.unwrap().y.to_radians(),
                    raw_vector_point.unwrap().z.to_radians(),
                ))
            },
            raw_vector_point,
            base_values,
            time,
            modifiers,
            easing,
        ))
    }

    fn interpolate_points(
        &self,
        points: &[Box<dyn BasePointData<Quat>>],
        l: usize,
        r: usize,
        time: f32,
        context: &BaseProviderContext,
    ) -> Quat {
        let point_l = points[l].get_point(context);
        let point_r = points[r].get_point(context);
        point_l.slerp(point_r, time)
    }

    fn get_points(&self) -> &Vec<Box<dyn BasePointData<Quat>>> {
        &self.points
    }
}

impl QuaternionPointDefinition {
    #[cfg(feature = "json")]
    pub fn new(value: &serde_json::Value, context: &BaseProviderContext) -> Self {
        let mut instance = Self { points: Vec::new() };
        instance.parse(value, context);
        instance
    }
}
