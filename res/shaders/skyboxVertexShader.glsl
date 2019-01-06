#version 400 core

in vec3 position;

out vec3 pass_tex_coords;

uniform mat4 projection_matrix;
uniform mat4 view_matrix;

void main(void) {
    gl_Position = projection_matrix * view_matrix * vec4(position, 1.0);
    pass_tex_coords = position;
}