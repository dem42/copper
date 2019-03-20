#version 400 core

in vec2 position;

uniform mat4 model_view_matrix;
uniform mat4 projection_matrix;

void main(void) {
    vec4 world_pos = projection_matrix * model_view_matrix * vec4(position, 0, 1);
    gl_Position = world_pos;
}