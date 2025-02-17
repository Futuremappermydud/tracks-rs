pub mod float_point_definition;


use std::{collections::HashMap, str::FromStr};

use serde_json::Value as JsonValue;

use crate::{
    easings::functions::Functions,
    modifiers::ModifierBase,
    modifiers::operation::Operation,
    point_data::BasePointData,
    values::{BaseValues, deserialize_values},
};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum GroupType {
    Value,
    Flag,
    Modifier,
}

// The combined PointDefinition trait (acting as both BasePointDefinition and the templated PointDefinition<T>)
pub trait PointDefinition {
    type Value: Default + Clone;

    // Required methods common to all definitions
    fn get_count(&self) -> usize;
    fn has_base_provider(&self) -> bool;
    fn interpolate_points(
        &self,
        points: &[Box<dyn BasePointData<Self::Value>>],
        l: usize,
        r: usize,
        time: f32,
    ) -> Self::Value;
    fn create_modifier(
        &self,
        values: Vec<Box<dyn BaseValues>>,
        modifiers: Vec<Box<dyn ModifierBase<Value = Self::Value>>>,
        operation: Operation,
    ) -> Box<dyn ModifierBase<Value = Self::Value>>;
    fn create_point_data(
        &self,
        values: Vec<Box<dyn BaseValues>>,
        flags: Vec<String>,
        modifiers: Vec<Box<dyn ModifierBase<Value = Self::Value>>>,
        easing: Functions,
    ) -> Box<dyn BasePointData<Self::Value>>;
    fn get_points_mut(&mut self) -> &mut Vec<Box<dyn BasePointData<Self::Value>>>;
    fn get_points(&self) -> &Vec<Box<dyn BasePointData<Self::Value>>>;

    #[cfg(feature = "json")]
    fn deserialize_modifier(&self, list: &JsonValue) -> Box<dyn ModifierBase<Value = Self::Value>> {
        let mut modifiers: Option<Vec<Box<dyn ModifierBase<Value = Self::Value>>>> = None;
        let mut operation: Option<Operation> = None;
        let mut values: Option<Vec<Box<dyn BaseValues>>> = None;

        // Group values similar to PointDefinition::group_values
        for group in Self::group_values(list) {
            match group.0 {
                GroupType::Value => {
                    values = Some(deserialize_values(&group.1));
                }
                GroupType::Modifier => {
                    modifiers = Some(
                        group
                            .1
                            .iter()
                            .map(|m| self.deserialize_modifier(m))
                            .collect(),
                    );
                }
                GroupType::Flag => {
                    operation = Some(Operation::from_str(group.1[0].as_str().unwrap()).unwrap());
                }
            }
        }

        // Validate required fields
        //let values = values.expect("No points found.");
        //let operation = operation.expect("No operation found.");

        // Create modifier with collected values
        self.create_modifier(
            values.unwrap(),
            modifiers.unwrap_or_default(),
            operation.unwrap(),
        )
    }

    // Shared parse implementation
    #[cfg(feature = "json")]
    fn parse(&mut self, value: &JsonValue) {
        // Expect an array of raw points
        if let Some(array) = value.as_array() {
            for raw_point in array {
                if raw_point.is_null() {
                    continue;
                }

                let mut easing = Functions::EaseLinear;
                let mut modifiers: Option<Vec<Box<dyn ModifierBase<Value = Self::Value>>>> = None;
                let mut flags: Option<Vec<String>> = None;
                let mut vals: Option<Vec<Box<dyn BaseValues>>> = None;

                // Group the values and flags. (Assuming each raw_point has a structure similar to the C++ JSON)
                for group in Self::group_values(raw_point) {
                    match group.0 {
                        GroupType::Value => {
                            vals = Some(deserialize_values(&group.1));
                        }
                        GroupType::Modifier => {
                            modifiers = Some(
                                group
                                    .1
                                    .iter()
                                    .map(|m| self.deserialize_modifier(m))
                                    .collect(),
                            );
                        }
                        GroupType::Flag => {
                            // Convert the group values (group.1) into a Vec<String>
                            let flags_vec: Vec<String> = group
                                .1
                                .iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect();

                            // Set the flags collected from the group.
                            flags = Some(flags_vec);

                            // Find the first flag starting with "ease" just like in the C# code.
                            if let Some(ref flags_inner) = flags
                                && let Some(easing_string) =
                                    flags_inner.iter().find(|flag| flag.starts_with("ease"))
                            {
                                easing = Functions::from_str(easing_string)
                                    .unwrap_or(Functions::EaseLinear);
                            }
                        }
                    }
                }

                // Create point data only if we have values
                if let Some(vs) = vals {
                    let point_data = self.create_point_data(
                        vs,
                        flags.unwrap_or_default(),
                        modifiers.unwrap_or_default(),
                        easing,
                    );
                    self.get_points_mut().push(point_data);
                }
            }
        }
    }

    // Binary search algorithm to find the relevant interval
    fn search_index(
        &self,
        points: &[Box<dyn BasePointData<Self::Value>>],
        time: f32,
    ) -> (usize, usize) {
        let mut l = 0;
        let mut r = points.len();

        while l < r - 1 {
            let m = (l + r) / 2;
            let point_time = points[m].get_time();
            if point_time < time {
                l = m;
            } else {
                r = m;
            }
        }

        (l, r)
    }

    // Helper method to group values from a JSON value.
    // In a more complete implementation, you'd examine the JSON structure.
    #[cfg(feature = "json")]
    fn group_values(value: &JsonValue) -> Vec<(GroupType, Vec<&JsonValue>)> {
        let mut result = Vec::new();
        if let Some(array) = value.as_array() {
            let values: Vec<&JsonValue> = array.iter().collect();
            let mut value_group = Vec::new();
            let mut flag_group = Vec::new();
            let mut modifier_group = Vec::new();

            for val in &values {
                if val.is_array() {
                    modifier_group.push(*val);
                } else if val.is_string() && !val.as_str().unwrap().starts_with("base") {
                    flag_group.push(*val);
                } else {
                    value_group.push(*val);
                }
            }

            if !value_group.is_empty() {
                result.push((GroupType::Value, value_group));
            }
            if !flag_group.is_empty() {
                result.push((GroupType::Flag, flag_group));
            }
            if !modifier_group.is_empty() {
                result.push((GroupType::Modifier, modifier_group));
            }
        }
        result
    }

    // The main interpolation method. Returns a tuple (interpolated value, is_last_point)
    fn interpolate(&self, time: f32) -> (Self::Value, bool) {
        let points = self.get_points();

        if points.is_empty() {
            return (Self::Value::default(), false);
        }

        let last_point = points.last().unwrap();
        if last_point.get_time() <= time {
            return (last_point.get_point(), true);
        }

        let first_point = points.first().unwrap();
        if first_point.get_time() >= time {
            return (first_point.get_point(), false);
        }

        let (l, r) = self.search_index(points, time);
        let point_l = &points[l];
        let point_r = &points[r];

        let normal_time = if point_r.get_time() - point_l.get_time() != 0.0 {
            (time - point_l.get_time()) / (point_r.get_time() - point_l.get_time())
        } else {
            0.0
        };

        let eased_time = point_r.get_easing().interpolate(normal_time);
        (self.interpolate_points(points, l, r, eased_time), false)
    }
}