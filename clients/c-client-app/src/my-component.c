#include "../my_world.h"
#include <stdlib.h>
#include <stdio.h>
#include <math.h>

typedef struct {
    float x;
    float y;
    float speed;
    float radius;
    char color[20];  // Assuming a maximum of 20 characters for the color string
} Particle;

Particle* particles = NULL;  // Pointer for dynamic array
int particleCount = 0;
int MAX_PARTICLES = 100;  // Set the maximum number of particles

unsigned int tick = 0;

void initializeParticles() {
    particles = (Particle*)malloc(MAX_PARTICLES * sizeof(Particle));
    if (particles == NULL) {
        my_world_string_t my_string;
        my_world_string_set(&my_string, "Memory allocation failed\n");

        levo_portal_my_imports_print(&my_string);
    
        exit(EXIT_FAILURE);
    }
}

void createParticles() {
    float canvas_width = 1200.0;  // TODO: pass from host
    tick++;
    if (tick % 10 == 0 && particleCount < MAX_PARTICLES) {
        particles[particleCount].x = ((float)rand() / RAND_MAX) * canvas_width;
        particles[particleCount].y = 0.0;
        particles[particleCount].speed = 500.0 + ((float)rand() / RAND_MAX) * 13.0;
        particles[particleCount].radius = 5.0 + ((float)rand() / RAND_MAX) * 5.0;

        const char* color = "white";
        for (int i = 0; i < sizeof(particles[particleCount].color) - 1 && color[i] != '\0'; ++i) {
            particles[particleCount].color[i] = color[i];
        }
        particles[particleCount].color[sizeof(particles[particleCount].color) - 1] = '\0';  // Ensure null-termination

        particleCount++;
    }
}

void updateParticles() {
    for (int i = 0; i < particleCount; i++) {
        particles[i].y -= particles[i].speed * levo_portal_my_imports_delta_seconds();
    }
}

void killParticles() {
    float canvas_height = 800.0;  // TODO: pass from host
    for (int i = 0; i < particleCount; i++) {
        if (particles[i].y < -canvas_height) {
            particles[i].y = 0.0;
        }
    }
}

void drawParticles() {
    levo_portal_my_imports_fill_style("royal_purple");
    levo_portal_my_imports_fill_rect(0.0, 0.0, 1200.0, 800.0);
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
        levo_portal_my_imports_fill_style(particles[i].color);
        levo_portal_my_imports_fill();
    }
}

void drawHeart() {
    levo_portal_my_imports_begin_path();
    levo_portal_my_imports_move_to(0.0, 0.0);
    levo_portal_my_imports_cubic_bezier_to(75.0, 75.0, 175.0, -50.0, 0.0, -150.0);
    levo_portal_my_imports_cubic_bezier_to(-175.0, -50.0, -75.0, 75.0, 0.0, 0.0);
    levo_portal_my_imports_close_path();

    my_world_string_t red_color;
    my_world_string_set(&red_color, "red");
    levo_portal_my_imports_fill_style(&red_color);
    levo_portal_my_imports_fill();

    my_world_string_free(&red_color);
}

void my_world_update() {
    my_world_string_t my_string;
    my_world_string_set(&my_string, "Happy New Year from C!");

    createParticles();
    updateParticles();
    killParticles();
    drawParticles();

    if (tick > 100) {
        drawHeart();
    }

    if (tick > 200) {
        levo_portal_my_imports_label(&my_string, 0.0, -200.0, 64.0, "white");
        levo_portal_my_imports_link("localhost/rust.wasm", "Go to rust.wasm", -100.0, -300.0, 32.0);
    }
}

void my_world_setup() {
    initializeParticles();
 
    my_world_string_t my_string;
    my_world_string_set(&my_string, "setup from guest (C) has been called");

    levo_portal_my_imports_print(&my_string);
}
