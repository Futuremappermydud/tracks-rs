#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
namespace Tracks {
#endif  // __cplusplus

/**
 * JSON FFI
 */
typedef enum JsonValueType {
  Number,
  Null,
  String,
  Array,
} JsonValueType;

typedef struct BaseFFIProviderValues BaseFFIProviderValues;

typedef struct BaseProviderContext BaseProviderContext;

typedef struct FloatPointDefinition FloatPointDefinition;

typedef struct QuaternionPointDefinition QuaternionPointDefinition;

typedef struct UpdatableProviderContext UpdatableProviderContext;

typedef struct Vector3PointDefinition Vector3PointDefinition;

typedef struct Vector4PointDefinition Vector4PointDefinition;

typedef struct JsonArray {
  const struct FFIJsonValue *elements;
  uintptr_t length;
} JsonArray;

typedef union JsonValueData {
  double number_value;
  const char *string_value;
  const struct JsonArray *array;
} JsonValueData;

typedef struct FFIJsonValue {
  enum JsonValueType value_type;
  union JsonValueData data;
} FFIJsonValue;

typedef struct WrappedValues {
  const float *values;
  uintptr_t length;
} WrappedValues;

typedef struct WrappedValues (*BaseFFIProvider)(const struct BaseProviderContext*, void*);

typedef struct FloatInterpolationResult {
  float value;
  bool is_last;
} FloatInterpolationResult;

typedef struct WrapVec3 {
  float x;
  float y;
  float z;
} WrapVec3;

typedef struct Vector3InterpolationResult {
  struct WrapVec3 value;
  bool is_last;
} Vector3InterpolationResult;

typedef struct WrapVec4 {
  float x;
  float y;
  float z;
  float w;
} WrapVec4;

typedef struct Vector4InterpolationResult {
  struct WrapVec4 value;
  bool is_last;
} Vector4InterpolationResult;

typedef struct WrapQuat {
  float x;
  float y;
  float z;
  float w;
} WrapQuat;

typedef struct QuaternionInterpolationResult {
  struct WrapQuat value;
  bool is_last;
} QuaternionInterpolationResult;



#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct FFIJsonValue tracks_create_json_number(double value);

struct FFIJsonValue tracks_create_json_string(const char *value);

struct FFIJsonValue tracks_create_json_array(const struct FFIJsonValue *elements, uintptr_t length);

void tracks_free_json_value(struct FFIJsonValue *json_value);

struct BaseFFIProviderValues *tracks_make_base_ffi_provider(const BaseFFIProvider *func,
                                                            void *user_value);

void tracks_dipose_base_ffi_provider(struct BaseFFIProviderValues *func);

/**
 * CONTEXT
 */
struct BaseProviderContext *tracks_make_base_provider_context(void);

void tracks_set_base_provider(struct BaseProviderContext *context,
                              const char *base,
                              float *values,
                              uintptr_t count,
                              bool quat);

/**
 *FLOAT POINT DEFINITION
 */
const struct FloatPointDefinition *tracks_make_float_point_definition(const struct FFIJsonValue *json,
                                                                      struct BaseProviderContext *context,
                                                                      struct UpdatableProviderContext *updatable_providers);

struct FloatInterpolationResult tracks_interpolate_float(const struct FloatPointDefinition *point_definition,
                                                         float time,
                                                         struct BaseProviderContext *context);

uintptr_t tracks_float_count(const struct FloatPointDefinition *point_definition);

bool tracks_float_has_base_provider(const struct FloatPointDefinition *point_definition);

/**
 *VECTOR3 POINT DEFINITION
 */
const struct Vector3PointDefinition *tracks_make_vector3_point_definition(const struct FFIJsonValue *json,
                                                                          struct BaseProviderContext *context,
                                                                          struct UpdatableProviderContext *updatable_providers);

struct Vector3InterpolationResult tracks_interpolate_vector3(const struct Vector3PointDefinition *point_definition,
                                                             float time,
                                                             struct BaseProviderContext *context);

uintptr_t tracks_vector3_count(const struct Vector3PointDefinition *point_definition);

bool tracks_vector3_has_base_provider(const struct Vector3PointDefinition *point_definition);

/**
 *VECTOR4 POINT DEFINITION
 */
const struct Vector4PointDefinition *tracks_make_vector4_point_definition(const struct FFIJsonValue *json,
                                                                          struct BaseProviderContext *context,
                                                                          struct UpdatableProviderContext *updatable_providers);

struct Vector4InterpolationResult tracks_interpolate_vector4(const struct Vector4PointDefinition *point_definition,
                                                             float time,
                                                             struct BaseProviderContext *context);

uintptr_t tracks_vector4_count(const struct Vector4PointDefinition *point_definition);

bool tracks_vector4_has_base_provider(const struct Vector4PointDefinition *point_definition);

/**
 *QUATERNION POINT DEFINITION
 */
const struct QuaternionPointDefinition *tracks_make_quat_point_definition(const struct FFIJsonValue *json,
                                                                          struct BaseProviderContext *context,
                                                                          struct UpdatableProviderContext *updatable_providers);

struct QuaternionInterpolationResult tracks_interpolate_quat(const struct QuaternionPointDefinition *point_definition,
                                                             float time,
                                                             struct BaseProviderContext *context);

uintptr_t tracks_quat_count(const struct QuaternionPointDefinition *point_definition);

bool tracks_quat_has_base_provider(const struct QuaternionPointDefinition *point_definition);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#ifdef __cplusplus
}  // namespace Tracks
#endif  // __cplusplus
