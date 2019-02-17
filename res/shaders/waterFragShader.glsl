#version 400 core

in vec4 clip_coords;
in vec2 tex_coords;

out vec4 final_color;

uniform sampler2D reflection_tex;
uniform sampler2D refraction_tex;
uniform sampler2D dudv_map;

uniform float wave_factor;

const float wave_strength = 0.02;

void main() {
    vec2 ndc_coords = clip_coords.xy / clip_coords.w;
    // move from [(-1,-1),(1,1)] rectangle to [(0,0),(1,1)]
    vec2 texture_coords = (ndc_coords + 1.0) / 2.0;
    vec2 refract_coords = vec2(texture_coords.x, texture_coords.y);
    vec2 reflect_coords = vec2(texture_coords.x, 1 - texture_coords.y);

    // this seems like a fancy way to distort the water
    vec2 distortion1 = (texture(dudv_map, vec2(tex_coords.x + wave_factor, tex_coords.y)).rg * 2.0 - 1.0) * wave_strength;
    vec2 distortion2 = (texture(dudv_map, vec2(-tex_coords.x + wave_factor, tex_coords.y + wave_factor)).rg * 2.0 - 1.0) * wave_strength;
    vec2 total_distortion = distortion1 + distortion2;

    reflect_coords += total_distortion;
    reflect_coords = clamp(reflect_coords, 0.001, 0.999);
    refract_coords += total_distortion;
    refract_coords = clamp(refract_coords, 0.001, 0.999);

    vec4 reflection_color = texture(reflection_tex, reflect_coords);
    vec4 refraction_color = texture(refraction_tex, refract_coords);

    final_color = mix(reflection_color, refraction_color, 0.5);
    // mix with a bit of blue/gree
    final_color = mix(final_color, vec4(0.0, 0.3, 0.5, 1.0), 0.2);    
}