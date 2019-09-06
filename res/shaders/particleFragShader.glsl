#version 400 core

in vec2 tex_coords1;
in vec2 tex_coords2;
in float blend;

layout(location = 0) out vec4 out_color;
layout(location = 1) out vec4 out_brightness_Color;

uniform sampler2D particle_texture;

void main(void) {
    vec4 color1 = texture(particle_texture, tex_coords1);
    vec4 color2 = texture(particle_texture, tex_coords2);

    out_color = mix(color1, color2, blend);
    out_brightness_Color = vec4(0.0);
}