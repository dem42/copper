#version 400 core

in vec2 texture_coords;

out vec4 out_color;

uniform sampler2D in_texture;

void main(void) {
    vec4 in_color = texture(in_texture, texture_coords);
    float luma = in_color.r * 0.2126 + in_color.g * 0.7152 + in_color.b * 0.0722;
    out_color = in_color * luma;
}