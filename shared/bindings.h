#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
namespace Tracks {
#endif  // __cplusplus

typedef struct BaseProviderContext BaseProviderContext;

typedef struct FloatPointDefinition FloatPointDefinition;

typedef struct QuaternionPointDefinition QuaternionPointDefinition;

typedef struct Vector3PointDefinition Vector3PointDefinition;

typedef struct Vector4PointDefinition Vector4PointDefinition;

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

/**
 * CONTEXT
 */
struct BaseProviderContext *tracks_make_base_provider_context(void);

/**
 *FLOAT POINT DEFINITION
 */
const struct FloatPointDefinition *tracks_make_float_point_definition(const char *json,
                                                                      struct BaseProviderContext *context);

struct FloatInterpolationResult tracks_interpolate_float(const struct FloatPointDefinition *point_definition,
                                                         float time,
                                                         struct BaseProviderContext *context);

uintptr_t tracks_float_count(const struct FloatPointDefinition *point_definition);

bool tracks_float_has_base_provider(const struct FloatPointDefinition *point_definition);

/**
 *VECTOR3 POINT DEFINITION
 */
const struct Vector3PointDefinition *tracks_make_vector3_point_definition(const char *json,
                                                                          struct BaseProviderContext *context);

struct Vector3InterpolationResult tracks_interpolate_vector3(const struct Vector3PointDefinition *point_definition,
                                                             float time,
                                                             struct BaseProviderContext *context);

uintptr_t tracks_vector3_count(const struct Vector3PointDefinition *point_definition);

bool tracks_vector3_has_base_provider(const struct Vector3PointDefinition *point_definition);

/**
 *VECTOR4 POINT DEFINITION
 */
const struct Vector4PointDefinition *tracks_make_vector4_point_definition(const char *json,
                                                                          struct BaseProviderContext *context);

struct Vector4InterpolationResult tracks_interpolate_vector4(const struct Vector4PointDefinition *point_definition,
                                                             float time,
                                                             struct BaseProviderContext *context);

uintptr_t tracks_vector4_count(const struct Vector4PointDefinition *point_definition);

bool tracks_vector4_has_base_provider(const struct Vector4PointDefinition *point_definition);

/**
 *QUATERNION POINT DEFINITION
 */
const struct QuaternionPointDefinition *tracks_make_quat_point_definition(const char *json,
                                                                          struct BaseProviderContext *context);

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
