use crate::point_definition::PointDefinition;
use crate::point_definition::float_point_definition::FloatPointDefinition;
use crate::values::base_provider_context::BaseProviderContext;
use serde_json::Value;
use std::ffi::{CStr, c_char};

//make context
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn tracks_make_base_provider_context() -> *const BaseProviderContext<'static> {
//   let context = Box::new(BaseProviderContext {
//     base_combo: &mut vec![0f32],
//   });
//   let context_ptr = Box::leak(context);
//   context_ptr
// }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_float_point_definition(
    json: *const c_char,
    context: *mut BaseProviderContext,
) -> *const FloatPointDefinition {
    let json_str = unsafe { CStr::from_ptr(json).to_str().unwrap() };
    let value: Value = serde_json::from_str(json_str).unwrap();
    let point_definition = Box::new(FloatPointDefinition::new(&value, unsafe { &*context }));
    let point_definition_ptr = Box::leak(point_definition);
    point_definition_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_float(
    point_definition: *const FloatPointDefinition,
    time: f32,
    context: *mut BaseProviderContext,
) -> f32 {
    let point_definition = unsafe { &*point_definition };
    point_definition.interpolate(time, unsafe { &*context }).0
}
