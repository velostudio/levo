#include "../my_world.h"
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef struct {
    float x;
    float y;
    float speed;
    float radius;
    char color[20];  // Assuming a maximum of 20 characters for the color string
} Particle;

Particle particles[100];  // Fixed-size array for particles
int particleCount = 0;

unsigned int tick = 0;

float heart_offset[2] = {0.0, 0.0};

void createParticles() {
    levo_portal_my_imports_size_t canvasSize;
    levo_portal_my_imports_canvas_size(&canvasSize);
    float canvas_width = canvasSize.width;

    tick++;
    
    levo_portal_my_imports_mouse_button_t leftMouseButton;
    leftMouseButton.tag = LEVO_PORTAL_MY_IMPORTS_MOUSE_BUTTON_LEFT;

    if (tick % 10 == 0 && particleCount < 100) {
        particles[particleCount].x = ((float)rand() / RAND_MAX) * canvas_width;
        particles[particleCount].y = 0.0;
        particles[particleCount].speed = 500.0 + ((float)rand() / RAND_MAX) * 13.0;
        particles[particleCount].radius = 5.0 + ((float)rand() / RAND_MAX) * 5.0;

        const char* color = "white";
        strncpy(particles[particleCount].color, color, sizeof(particles[particleCount].color) - 1);
        particles[particleCount].color[sizeof(particles[particleCount].color) - 1] = '\0';  // Ensure null-termination

        if (levo_portal_my_imports_mouse_button_pressed(&leftMouseButton)) {
            levo_portal_my_imports_position_t cursor_position;
            levo_portal_my_imports_cursor_position(&cursor_position);
            particles[particleCount].x = cursor_position.x;
            particles[particleCount].y = cursor_position.y;
        }

        particleCount++;
    }
}

void updateParticles() {
    for (int i = 0; i < particleCount; i++) {
        particles[i].y -= particles[i].speed * levo_portal_my_imports_delta_seconds();
    }
}

void killParticles() {
    levo_portal_my_imports_size_t canvasSize;
    levo_portal_my_imports_canvas_size(&canvasSize);
    float canvas_height = canvasSize.height;
    for (int i = 0; i < particleCount; i++) {
        if (particles[i].y < -canvas_height) {
            particles[i].y = 0.0;
        }
    }
}

void drawParticles() {
    my_world_string_t fill_style;
    my_world_string_set(&fill_style, "royal_purple");
    levo_portal_my_imports_fill_style(&fill_style);
    levo_portal_my_imports_size_t canvasSize;
    levo_portal_my_imports_canvas_size(&canvasSize);
    float canvas_height = canvasSize.height;
    levo_portal_my_imports_fill_rect(0.0, 0.0, canvasSize.width, canvasSize.height);

    for (int i = 0; i < particleCount; i++) {
        levo_portal_my_imports_begin_path();
        levo_portal_my_imports_arc(
            particles[i].x,
            particles[i].y,
            particles[i].radius,
            2.0 * M_PI,
            0.0
        );
        levo_portal_my_imports_close_path();
        my_world_string_t fill_style;
        my_world_string_set(&fill_style, particles[i].color);
        levo_portal_my_imports_fill_style(&fill_style);
        levo_portal_my_imports_fill();
    }
}

void drawHeart(float x_offset, float y_offset) {
    levo_portal_my_imports_begin_path();
    levo_portal_my_imports_move_to(x_offset, y_offset);
    levo_portal_my_imports_cubic_bezier_to(x_offset + 75.0, y_offset + 75.0, x_offset + 175.0, y_offset - 50.0, x_offset, y_offset - 150.0);
    levo_portal_my_imports_cubic_bezier_to(x_offset - 175.0, y_offset - 50.0, x_offset - 75.0, y_offset + 75.0, x_offset, y_offset);
    levo_portal_my_imports_close_path();

    my_world_string_t fill_style;
    my_world_string_set(&fill_style, "red");
    levo_portal_my_imports_fill_style(&fill_style);
 
    levo_portal_my_imports_fill();
    my_world_string_free(&fill_style);
}

void my_world_update() {
    createParticles();
    updateParticles();
    killParticles();
    drawParticles();

    if (tick > 100) {
        float heart_speed = 222.0;
    
        if (levo_portal_my_imports_key_pressed(LEVO_PORTAL_MY_IMPORTS_KEY_CODE_LEFT)) {
            heart_offset[0] -= heart_speed * levo_portal_my_imports_delta_seconds();
        }
    
        if (levo_portal_my_imports_key_pressed(LEVO_PORTAL_MY_IMPORTS_KEY_CODE_RIGHT)) {
            heart_offset[0] += heart_speed * levo_portal_my_imports_delta_seconds();
        }
    
        if (levo_portal_my_imports_key_pressed(LEVO_PORTAL_MY_IMPORTS_KEY_CODE_UP)) {
            heart_offset[1] += heart_speed * levo_portal_my_imports_delta_seconds();
        }
    
        if (levo_portal_my_imports_key_pressed(LEVO_PORTAL_MY_IMPORTS_KEY_CODE_DOWN)) {
            heart_offset[1] -= heart_speed * levo_portal_my_imports_delta_seconds();
        }
    
        drawHeart(heart_offset[0], heart_offset[1]);
    }
    
    if (tick > 200) {
        my_world_string_t my_string;
        my_world_string_set(&my_string, "Happy New Year from C!");
    
        my_world_string_t label_color;
        my_world_string_set(&label_color, "white");

        my_world_string_t link_url;
        my_world_string_set(&link_url, "localhost/go.wasm");

        my_world_string_t link_name;
        my_world_string_set(&link_name, "Go to go.wasm");

        levo_portal_my_imports_label(&my_string, 0.0, -200.0, 64.0, &label_color);
        levo_portal_my_imports_link(&link_url, &link_name, -100.0, -300.0, 32.0);
    }
}

void my_world_setup() {
    my_world_string_t my_string;
    my_world_string_set(&my_string, "setup from guest (C) has been called");

    levo_portal_my_imports_print(&my_string);
}
