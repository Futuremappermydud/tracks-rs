use crate::point_definition::float_point_definition::FloatPointDefinition;
use crate::point_definition::quaternion_point_definition::QuaternionPointDefinition;
use crate::point_definition::vector4_point_definition::Vector4PointDefinition;
use crate::point_definition::{PointDefinition, vector3_point_definition::Vector3PointDefinition};
use crate::values::base_provider_context::BaseProviderContext;
use serde_json::Value;
use std::ffi::{CStr, c_char};

#[repr(C)]
pub struct WrapVec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
pub struct WrapVec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[repr(C)]
pub struct WrapQuat {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[repr(C)]
pub struct FloatInterpolationResult {
    value: f32,
    is_last: bool,
}

#[repr(C)]
pub struct Vector3InterpolationResult {
    value: WrapVec3,
    is_last: bool,
}

#[repr(C)]
pub struct Vector4InterpolationResult {
    value: WrapVec4,
    is_last: bool,
}

#[repr(C)]
pub struct QuaternionInterpolationResult {
    value: WrapQuat,
    is_last: bool,
}

/// CONTEXT
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_base_provider_context() -> *mut BaseProviderContext {
  let context = Box::new(BaseProviderContext::new());
  let context_ptr = Box::leak(context);
  context_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_set_base_provider(context: *mut BaseProviderContext, base: *const c_char, values: &[f32]) {
  let base_str = unsafe { CStr::from_ptr(base).to_str().unwrap() };
  let context = unsafe { &mut *context };
  context.set_values(base_str, values.to_vec());
}

///FLOAT POINT DEFINITION
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
) -> FloatInterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    FloatInterpolationResult { value, is_last }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_float_count(point_definition: *const FloatPointDefinition) -> usize {
  let point_definition = unsafe { &*point_definition };
  point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_float_has_base_provider(point_definition: *const FloatPointDefinition) -> bool {
  let point_definition = unsafe { &*point_definition };
  point_definition.has_base_provider()
}

///VECTOR3 POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_vector3_point_definition(
    json: *const c_char,
    context: *mut BaseProviderContext,
) -> *const Vector3PointDefinition {
    let json_str = unsafe { CStr::from_ptr(json).to_str().unwrap() };
    let value: Value = serde_json::from_str(json_str).unwrap();
    let point_definition = Box::new(Vector3PointDefinition::new(&value, unsafe { &*context }));
    let point_definition_ptr = Box::leak(point_definition);
    point_definition_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_vector3(
    point_definition: *const Vector3PointDefinition,
    time: f32,
    context: *mut BaseProviderContext,
) -> Vector3InterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    Vector3InterpolationResult {
        value: WrapVec3 {
            x: value.x,
            y: value.y,
            z: value.z,
        },
        is_last,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector3_count(point_definition: *const Vector3PointDefinition) -> usize {
  let point_definition = unsafe { &*point_definition };
  point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector3_has_base_provider(point_definition: *const Vector3PointDefinition) -> bool {
  let point_definition = unsafe { &*point_definition };
  point_definition.has_base_provider()
}

///VECTOR4 POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_vector4_point_definition(
    json: *const c_char,
    context: *mut BaseProviderContext,
) -> *const Vector4PointDefinition {
    let json_str = unsafe { CStr::from_ptr(json).to_str().unwrap() };
    let value: Value = serde_json::from_str(json_str).unwrap();
    let point_definition = Box::new(Vector4PointDefinition::new(&value, unsafe { &*context }));
    let point_definition_ptr = Box::leak(point_definition);
    point_definition_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_vector4(
    point_definition: *const Vector4PointDefinition,
    time: f32,
    context: *mut BaseProviderContext,
) -> Vector4InterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    Vector4InterpolationResult {
        value: WrapVec4 {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        },
        is_last,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector4_count(point_definition: *const Vector4PointDefinition) -> usize {
  let point_definition = unsafe { &*point_definition };
  point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector4_has_base_provider(point_definition: *const Vector4PointDefinition) -> bool {
  let point_definition = unsafe { &*point_definition };
  point_definition.has_base_provider()
}

///QUATERNION POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_quat_point_definition(
    json: *const c_char,
    context: *mut BaseProviderContext,
) -> *const QuaternionPointDefinition {
    let json_str = unsafe { CStr::from_ptr(json).to_str().unwrap() };
    let value: Value = serde_json::from_str(json_str).unwrap();
    let point_definition = Box::new(QuaternionPointDefinition::new(&value, unsafe { &*context }));
    let point_definition_ptr = Box::leak(point_definition);
    point_definition_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_quat(
    point_definition: *const QuaternionPointDefinition,
    time: f32,
    context: *mut BaseProviderContext,
) -> QuaternionInterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    QuaternionInterpolationResult {
        value: WrapQuat {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        },
        is_last,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_quat_count(point_definition: *const QuaternionPointDefinition) -> usize {
  let point_definition = unsafe { &*point_definition };
  point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_quat_has_base_provider(point_definition: *const QuaternionPointDefinition) -> bool {
  let point_definition = unsafe { &*point_definition };
  point_definition.has_base_provider()
}

