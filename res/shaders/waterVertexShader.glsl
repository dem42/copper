#version 400 core

in vec3 position;

out vec4 clip_coords;
out vec2 tex_coords;

uniform mat4 projection_matrix;
uniform mat4 view_matrix;
uniform mat4 transform_matrix;

const float tiling = 6.0;

void main() {
    clip_coords = projection_matrix * view_matrix * transform_matrix * vec4(position, 1.0);
    gl_Position = clip_coords;
    tex_coords = (position.xz / 2.0 + 0.5) * tiling;
}