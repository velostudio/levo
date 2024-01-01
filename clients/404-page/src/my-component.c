#include "../my_world.h"

void my_world_update() {
    my_world_string_t my_string;
    my_world_string_set(&my_string, "404 Wasm App Not Found"); // TODO: think about how to print requested resource path too 
    
    my_world_string_t label_color;
    my_world_string_set(&label_color, "white");

    levo_portal_my_imports_label(&my_string, 0.0, 0.0, 64.0, &label_color);
}

void my_world_setup() {}

