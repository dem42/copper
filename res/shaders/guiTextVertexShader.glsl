#version 400 core

in vec2 position;
in vec2 tex_coord;

out vec2 pass_tex_coord;

void main(void) {
    gl_Position = vec4(position, 0.0, 1.0);

    pass_tex_coord = position;
}