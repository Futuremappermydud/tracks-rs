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

typedef struct Vector3PointDefinition Vector3PointDefinition;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

const struct FloatPointDefinition *tracks_make_float_point_definition(const char *json,
                                                                      struct BaseProviderContext *context);

float tracks_interpolate_float(const struct FloatPointDefinition *point_definition,
                               float time,
                               struct BaseProviderContext *context);

const struct Vector3PointDefinition *tracks_make_vector3_point_definition(const char *json,
                                                                          struct BaseProviderContext *context);

Vec3 tracks_interpolate_vector3(const struct Vector3PointDefinition *point_definition,
                                float time,
                                struct BaseProviderContext *context);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#ifdef __cplusplus
}  // namespace Tracks
#endif  // __cplusplus
