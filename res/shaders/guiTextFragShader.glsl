#version 400 core

in vec2 pass_tex_coord;

out vec4 out_color;

uniform sampler2D font_texture;
uniform vec3 color;

void main(void) {
    out_color = vec4(color, texture(font_texture, pass_tex_coord).a);
}