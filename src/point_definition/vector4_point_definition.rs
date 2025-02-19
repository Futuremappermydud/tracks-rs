use glam::{FloatExt, Vec4};
use palette::{Hsv, IntoColor, LinSrgb, RgbHue, rgb::Rgb};

use crate::{
    easings::functions::Functions,
    modifiers::{operation::Operation, vector4_modifier::Vector4Modifier, ModifierBase},
    point_data::{vector4_point_data::Vector4PointData, BasePointData},
    values::{base_provider_context::BaseProviderContext, r#static::StaticValues, AbstractValueProvider, ValueProvider},
};

use super::PointDefinition;

pub struct Vector4PointDefinition {
    points: Vec<Box<dyn BasePointData<Vec4>>>,
}

pub fn lerp_hsv_vec4(color1: Vec4, color2: Vec4, time: f32) -> Vec4 {
    // Convert RGBA to HSV
    let hsv1: Hsv<f32> = Rgb::new(color1.x, color1.y, color1.z).into_color();
    let hsv2: Hsv<f32> = Rgb::new(color2.x, color2.y, color2.z).into_color();

    // Lerp HSV components
    let h = RgbHue::from_radians(
        hsv1.hue
            .into_raw_radians()
            .lerp(hsv2.hue.into_raw_radians(), time),
    );
    let s = hsv1.saturation.lerp(hsv2.saturation, time);
    let v = hsv1.value.lerp(hsv2.value, time);

    // Convert back to RGB
    let rgb: LinSrgb<f32> = Hsv::new(h, s, v).into_color();

    // Lerp alpha
    let alpha = color1.w * (1.0 - time) + color2.w * time;

    // Return the new Vec4
    Vec4::new(rgb.red, rgb.green, rgb.blue, alpha)
}

impl PointDefinition for Vector4PointDefinition {
    type Value = Vec4;

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
        modifiers: Vec<Box<dyn ModifierBase<Value = Vec4>>>,
        operation: Operation,
        context: &BaseProviderContext,
    ) -> Box<dyn ModifierBase<Value = Vec4>> {
        let mut raw_point: Option<Vec4> = None;
        let base_values = if values.len() == 1 {
            if let Some(static_val) = values[0].as_ref().as_any().downcast_ref::<StaticValues>() {
                if static_val.values(context).len() == 4 {
                    raw_point = Some(Vec4::new(
                        static_val.values(context)[0],
                        static_val.values(context)[1],
                        static_val.values(context)[2],
                        static_val.values(context)[3],
                    ));
                    None
                } else {
                    let count: usize = values.iter().map(|v| v.values(context).len()).sum();
                    assert_eq!(count, 4, "Vector4 modifier point must have 4 numbers");
                    Some(values)
                }
            } else {
                let count: usize = values.iter().map(|v| v.values(context).len()).sum();
                assert_eq!(count, 4, "Vector4 modifier point must have 4 numbers");
                Some(values)
            }
        } else {
            let count: usize = values.iter().map(|v| v.values(context).len()).sum();
            assert_eq!(count, 4, "Vector4 modifier point must have 4 numbers");
            Some(values)
        };
        Box::new(Vector4Modifier::new(
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
        modifiers: Vec<Box<dyn ModifierBase<Value = Vec4>>>,
        easing: Functions,
        context: &BaseProviderContext,
    ) -> Box<dyn BasePointData<Vec4>> {
        let mut raw_point: Option<Vec4> = None;
        let time: f32;
        let base_values = if values.len() == 1 {
            if let Some(static_val) = values[0].as_ref().as_any().downcast_ref::<StaticValues>() {
                if static_val.values(context).len() == 5 {
                    raw_point = Some(Vec4::new(
                        static_val.values(context)[0],
                        static_val.values(context)[1],
                        static_val.values(context)[2],
                        static_val.values(context)[3],
                    ));
                    time = static_val.values(context)[4];
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
            if count != 5 {
                eprintln!("Vector4 point must have 4 numbers");
            }
            time = values
                .last()
                .and_then(|v| v.values(context).last().copied())
                .unwrap_or(0.0);
            Some(values)
        };
        Box::new(Vector4PointData::new(
            raw_point,
            base_values,
            flags.iter().any(|f| f == "lerpHSV"),
            time,
            modifiers,
            easing,
        ))
    }

    fn interpolate_points(
        &self,
        points: &[Box<dyn BasePointData<Vec4>>],
        l: usize,
        r: usize,
        time: f32,
        context: &BaseProviderContext,
    ) -> Vec4 {
        let point_data_r = points[r]
            .as_any()
            .downcast_ref::<Vector4PointData>()
            .unwrap();

        let point_l = points[l].get_point(context);
        let point_r = points[r].get_point(context);

        if !point_data_r.hsv_lerp {
            point_l.lerp(point_r, time)
        } else {
            lerp_hsv_vec4(point_l, point_r, time)
        }
    }

    fn get_points(&self) -> &Vec<Box<dyn BasePointData<Vec4>>> {
        &self.points
    }
}

impl Vector4PointDefinition {
    #[cfg(feature = "json")]
    pub fn new(value: &serde_json::Value, context: &BaseProviderContext) -> Self {
        let mut instance = Self { points: Vec::new() };
        instance.parse(value, context);
        instance
    }
}
