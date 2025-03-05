use crate::point_definition::float_point_definition::FloatPointDefinition;
use crate::point_definition::quaternion_point_definition::QuaternionPointDefinition;
use crate::point_definition::vector4_point_definition::Vector4PointDefinition;
use crate::point_definition::{PointDefinition, vector3_point_definition::Vector3PointDefinition};
use crate::values::base_ffi::{BaseFFIProvider, BaseFFIProviderValues};
use crate::values::base_provider_context::{BaseProviderContext, UpdatableProviderContext};
use crate::values::value::BaseValue;
use std::ffi::{CStr, c_char};
use std::os::raw::c_void;
use std::slice;
use tracing::info;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WrapVec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WrapVec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WrapQuat {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum WrapBaseValueType {
    Vec3 = 0,
    Quat = 1,
    Vec4 = 2,
    Float = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union WrapBaseValueUnion {
    vec3: WrapVec3,
    quat: WrapQuat,
    vec4: WrapVec4,
    float: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WrapBaseValue {
    ty: WrapBaseValueType,
    value: WrapBaseValueUnion,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WrappedValues {
    pub values: *const f32,
    pub length: usize,
}

#[repr(C)]
pub struct FloatInterpolationResult {
    pub value: f32,
    pub is_last: bool,
}

#[repr(C)]
pub struct Vector3InterpolationResult {
    pub value: WrapVec3,
    pub is_last: bool,
}

#[repr(C)]
pub struct Vector4InterpolationResult {
    pub value: WrapVec4,
    pub is_last: bool,
}

#[repr(C)]
pub struct QuaternionInterpolationResult {
    pub value: WrapQuat,
    pub is_last: bool,
}

/// JSON FFI
#[repr(C)]
#[derive(Debug)]
pub enum JsonValueType {
    Number,
    Null,
    String,
    Array,
}

#[repr(C)]
pub struct FFIJsonValue {
    pub value_type: JsonValueType,
    pub data: JsonValueData,
}

#[repr(C)]
pub union JsonValueData {
    pub number_value: f64,
    pub string_value: *const c_char,
    pub array: *const JsonArray,
}

#[repr(C)]
pub struct JsonArray {
    pub elements: *const FFIJsonValue,
    pub length: usize,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_create_json_number(value: f64) -> FFIJsonValue {
    FFIJsonValue {
        value_type: JsonValueType::Number,
        data: JsonValueData {
            number_value: value,
        },
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_create_json_string(value: *const c_char) -> FFIJsonValue {
    FFIJsonValue {
        value_type: JsonValueType::String,
        data: JsonValueData {
            string_value: value,
        },
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_create_json_array(
    elements: *const FFIJsonValue,
    length: usize,
) -> FFIJsonValue {
    // Allocate JsonArray on the heap so it persists after function returns
    let array = Box::new(JsonArray { elements, length });

    // Leak the box to keep it alive (caller is responsible for freeing)
    let array_ptr = Box::into_raw(array);

    FFIJsonValue {
        value_type: JsonValueType::Array,
        data: JsonValueData { array: array_ptr },
    }
}

/// Convert the FFI JsonValue to a serde_json::Value
unsafe fn convert_json_value_to_serde(json_value: *const FFIJsonValue) -> serde_json::Value {
    if json_value.is_null() {
        return serde_json::Value::Null;
    }

    let json_value = unsafe { &*json_value };
    match json_value.value_type {
        JsonValueType::Null => serde_json::Value::Null,
        JsonValueType::Number => serde_json::Value::Number(
            serde_json::Number::from_f64(unsafe { json_value.data.number_value }).unwrap(),
        ),
        JsonValueType::String => {
            let c_str = unsafe { CStr::from_ptr(json_value.data.string_value) };
            let str_slice = c_str.to_str().unwrap_or_default();
            serde_json::Value::String(str_slice.to_owned())
        }
        JsonValueType::Array => {
            let array_ptr = unsafe { json_value.data.array };
            if array_ptr.is_null() {
                return serde_json::Value::Array(Vec::new());
            }

            let array = unsafe { &*array_ptr };

            // Validate array length - prevent unreasonable allocations
            // 10 million elements should be more than enough for any reasonable JSON array
            // while preventing buffer overflows from corrupted memory
            const MAX_SAFE_ARRAY_LENGTH: usize = 10_000_000;

            if array.elements.is_null() || array.length == 0 || array.length > MAX_SAFE_ARRAY_LENGTH
            {
                println!(
                    "Invalid array length or null elements pointer: {}",
                    array.length
                );
                return serde_json::Value::Array(Vec::new());
            }

            // Create a safe slice from the raw parts
            let elements = unsafe { slice::from_raw_parts(array.elements, array.length) };
            let mut json_array = Vec::new(); // Don't pre-allocate with potentially corrupted capacity

            for element in elements.iter() {
                json_array.push(unsafe { convert_json_value_to_serde(element) });
            }

            serde_json::Value::Array(json_array)
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_free_json_value(json_value: *mut FFIJsonValue) {
    // Free the memory allocated for the JsonValue
    // This is a simple implementation that doesn't handle nested structures
    // For a complete implementation, you would need to recursively free all nested elements
    if !json_value.is_null() {
        drop(unsafe { Box::from_raw(json_value) });
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_base_ffi_provider(
    func: *const BaseFFIProvider,
    user_value: *mut c_void,
) -> *mut BaseFFIProviderValues {
    assert!(!func.is_null());

    let context = Box::new(BaseFFIProviderValues::new(func, user_value));
    let context_ptr = Box::leak(context);
    context_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_dipose_base_ffi_provider(func: *mut BaseFFIProviderValues) {
    assert!(!func.is_null());

    // destroy
    unsafe {
        let _ = Box::from_raw(func);
    };
}

/// CONTEXT
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_base_provider_context() -> *mut BaseProviderContext {
    let context = Box::new(BaseProviderContext::new());
    let context_ptr = Box::leak(context);
    context_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_set_base_provider(
    context: *mut BaseProviderContext,
    base: *const c_char,
    values: *mut f32,
    count: usize,
    quat: bool,
) {
    let base_str = unsafe { CStr::from_ptr(base).to_str().unwrap() };
    let context = unsafe { &mut *context };
    context.set_values(base_str, unsafe {
        let v = slice::from_raw_parts(values, count);
        info!("v: {} {:?}", base_str, v);
        BaseValue::from_slice(v, quat)
    });
}

///FLOAT POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_float_point_definition(
    json: *const FFIJsonValue,
    context: *mut BaseProviderContext,
    updatable_providers: *mut UpdatableProviderContext,
) -> *const FloatPointDefinition {
    let value = unsafe { convert_json_value_to_serde(json) };
    let point_definition = Box::new(FloatPointDefinition::new(
        value,
        unsafe { &mut *context },
        unsafe { &mut *updatable_providers },
    ));
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
pub unsafe extern "C" fn tracks_float_count(
    point_definition: *const FloatPointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_float_has_base_provider(
    point_definition: *const FloatPointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}

///VECTOR3 POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_vector3_point_definition(
    json: *const FFIJsonValue,
    context: *mut BaseProviderContext,
    updatable_providers: *mut UpdatableProviderContext,
) -> *const Vector3PointDefinition {
    let value = unsafe { convert_json_value_to_serde(json) };
    let point_definition = Box::new(Vector3PointDefinition::new(
        value,
        unsafe { &mut *context },
        unsafe { &mut *updatable_providers },
    ));
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
pub unsafe extern "C" fn tracks_vector3_count(
    point_definition: *const Vector3PointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector3_has_base_provider(
    point_definition: *const Vector3PointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}

///VECTOR4 POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_vector4_point_definition(
    json: *const FFIJsonValue,
    context: *mut BaseProviderContext,
    updatable_providers: *mut UpdatableProviderContext,
) -> *const Vector4PointDefinition {
    let value = unsafe { convert_json_value_to_serde(json) };
    let point_definition = Box::new(Vector4PointDefinition::new(
        value,
        unsafe { &mut *context },
        unsafe { &mut *updatable_providers },
    ));
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
pub unsafe extern "C" fn tracks_vector4_count(
    point_definition: *const Vector4PointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector4_has_base_provider(
    point_definition: *const Vector4PointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}

///QUATERNION POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_quat_point_definition(
    json: *const FFIJsonValue,
    context: *mut BaseProviderContext,
    updatable_providers: *mut UpdatableProviderContext,
) -> *const QuaternionPointDefinition {
    let value = unsafe { convert_json_value_to_serde(json) };
    let point_definition = Box::new(QuaternionPointDefinition::new(
        value,
        unsafe { &mut *context },
        unsafe { &mut *updatable_providers },
    ));
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
pub unsafe extern "C" fn tracks_quat_count(
    point_definition: *const QuaternionPointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_quat_has_base_provider(
    point_definition: *const QuaternionPointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}
