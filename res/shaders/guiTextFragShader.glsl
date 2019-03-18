#version 400 core

in vec2 pass_tex_coord;

out vec4 out_color;

uniform sampler2D font_texture;

const vec3 text_color = vec3(1.0, 0.0, 0.0);

void main(void) {
    out_color = vec4(text_color, texture(font_texture, pass_tex_coord).a);
}