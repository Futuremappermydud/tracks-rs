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

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

const struct BaseProviderContext *tracks_make_base_provider_context(void);

const struct FloatPointDefinition *tracks_make_float_point_definition(const char *json,
                                                                      struct BaseProviderContext *context);

float tracks_interpolate_float(const struct FloatPointDefinition *point_definition, float time);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#ifdef __cplusplus
}  // namespace Tracks
#endif  // __cplusplus
