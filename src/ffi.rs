use std::ffi::{c_char, CStr};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::point_definition::float_point_definition::FloatPointDefinition;
// use crate::point_definition::point_definition_type::PointDefinitionType;
use crate::point_definition::PointDefinition;
use serde_json::Value;

// pub struct PointDefinitionContainer {
//   pub points: HashMap<String, PointDefinitionType>,
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn tracks_make_point_manager_container() -> *mut PointDefinitionContainer {
//   let manager = Box::new(PointDefinitionContainer { points: HashMap::new() });
//   Box::leak(manager)
// }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_float_point_definition(json: *const c_char/*, container: *mut PointDefinitionContainer*/) -> *const FloatPointDefinition {
  let json_str = unsafe { CStr::from_ptr(json).to_str().unwrap() };
  let value: Value = serde_json::from_str(json_str).unwrap();
  let point_definition = Box::new(FloatPointDefinition::new(&value));
  let point_definition_ptr = Box::leak(point_definition);
  // let point_definition_type = PointDefinitionType::Float(point_definition_ptr);
  
  /*let mut hasher = DefaultHasher::new();
  json_str.hash(&mut hasher);
  let hash = hasher.finish().to_string();
  
  unsafe {
    (*container).points.insert(hash, point_definition_type);
  }*/
  point_definition_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_float(point_definition: *const FloatPointDefinition, time: f32) -> f32 {
  let point_definition = unsafe { &*point_definition };
  point_definition.interpolate(time).0
}

