#version 400 core

in vec2 texture_coords;

out vec4 out_color;

uniform sampler2D in_texture;
uniform sampler2D brightness_tex;

void main(void) {
    vec4 in_color = texture(in_texture, texture_coords);
    vec4 brightness = texture(brightness_tex, texture_coords);
    out_color = in_color + brightness;    
}