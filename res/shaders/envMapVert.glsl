#version 400 core

in vec3 position;
in vec2 tex_coord;
in vec3 normal;

out vec2 pass_tex_coord;

uniform mat4 mvp_matrix;

void main(void) {
    gl_Position = mvp_matrix * vec4(position, 1.0);

    pass_tex_coord = tex_coord;
}