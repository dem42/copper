#version 400 core

in vec3 position;

out vec4 clip_coords;

uniform mat4 projection_matrix;
uniform mat4 view_matrix;
uniform mat4 transform_matrix;

void main() {
    clip_coords = projection_matrix * view_matrix * transform_matrix * vec4(position, 1.0);
    gl_Position = clip_coords;
}