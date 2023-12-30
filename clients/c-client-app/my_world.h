// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
#ifndef __BINDINGS_MY_WORLD_H
#define __BINDINGS_MY_WORLD_H
#ifdef __cplusplus
extern "C" {
#endif

#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct {
  uint8_t*ptr;
  size_t len;
} my_world_string_t;

// Imported Functions from `levo:portal/my-imports`
extern void levo_portal_my_imports_print(my_world_string_t *msg);
extern void levo_portal_my_imports_fill_style(my_world_string_t *color);
extern void levo_portal_my_imports_fill_rect(float x, float y, float width, float height);
extern void levo_portal_my_imports_begin_path(void);
extern void levo_portal_my_imports_move_to(float x, float y);
extern void levo_portal_my_imports_cubic_bezier_to(float x1, float y1, float x2, float y2, float x3, float y3);
extern void levo_portal_my_imports_arc(float x, float y, float radius, float sweep_angle, float x_rotation);
extern void levo_portal_my_imports_close_path(void);
extern void levo_portal_my_imports_fill(void);
extern void levo_portal_my_imports_label(my_world_string_t *text, float x, float y, float size, my_world_string_t *color);
extern void levo_portal_my_imports_link(my_world_string_t *url, my_world_string_t *text, float x, float y, float size);
extern float levo_portal_my_imports_delta_seconds(void);

// Exported Functions from `my-world`
void my_world_update(void);
void my_world_setup(void);

// Helper Functions

// Transfers ownership of `s` into the string `ret`
void my_world_string_set(my_world_string_t *ret, char*s);

// Creates a copy of the input nul-terminate string `s` and
// stores it into the component model string `ret`.
void my_world_string_dup(my_world_string_t *ret, const char*s);

// Deallocates the string pointed to by `ret`, deallocating
// the memory behind the string.
void my_world_string_free(my_world_string_t *ret);

#ifdef __cplusplus
}
#endif
#endif
