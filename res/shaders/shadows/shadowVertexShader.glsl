#version 400 core

in vec3 pos;
in vec2 in_texture_coords;

out vec2 texture_coords;

uniform mat4 mvp_matrix;

void main(void) {
    texture_coords = in_texture_coords;
    gl_Position = mvp_matrix * vec4(pos, 1.0);
}