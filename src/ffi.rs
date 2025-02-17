use std::ffi::{c_char, CStr};
use crate::point_definition::float_point_definition::FloatPointDefinition;
use crate::point_definition::PointDefinition;
use serde_json::Value;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_float_point_definition(json: *const c_char) -> *const FloatPointDefinition {
  let json_str = unsafe { CStr::from_ptr(json).to_str().unwrap() };
  let value: Value = serde_json::from_str(json_str).unwrap();
  let point_definition = Box::new(FloatPointDefinition::new(&value));
  let point_definition_ptr = Box::leak(point_definition);
  point_definition_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_float(point_definition: *const FloatPointDefinition, time: f32) -> f32 {
  let point_definition = unsafe { &*point_definition };
  point_definition.interpolate(time).0
}

