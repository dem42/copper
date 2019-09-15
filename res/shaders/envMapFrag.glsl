#version 400 core

in vec2 pass_tex_coord;

out vec4 out_color;

uniform sampler2D in_texture;

void main(void) {
    out_color = texture(in_texture, pass_tex_coord);
}