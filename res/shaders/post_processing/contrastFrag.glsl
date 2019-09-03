#version 400 core

in vec2 texture_coords;

out vec4 gl_Color;

uniform sampler2D color_texture;

const float contrast = 0.3;

void main(void) {
    vec4 color = texture(color_texture, texture_coords);
    // scale to [-0.5, 0.5] then increase the contrast .. then translate back
    color.rgb = (color.rgb - 0.5) * (1.0 + contrast) + 0.5;
    gl_Color = color;
}