// my-component.c

#include "../my_world.h"

void my_world_update() {
    my_world_string_t my_string;
    my_world_string_set(&my_string, "Happy New Year from C!");

    float x = 0.0f;
    float y = -200.0f;
    float size = 64.0f;

    my_world_string_t color;
    my_world_string_set(&color, "white");

    levo_portal_my_imports_label(&my_string, x, y, size, &color);
}

void my_world_setup() {
    my_world_string_t my_string;
    my_world_string_set(&my_string, "setup from guest (C) has been called");

    levo_portal_my_imports_print(&my_string);
}
