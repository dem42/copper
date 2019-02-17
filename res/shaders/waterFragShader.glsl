#version 400 core

in vec4 clip_coords;

out vec4 final_color;

uniform sampler2D reflection_tex;
uniform sampler2D refraction_tex;

void main() {
    vec2 ndc_coords = clip_coords.xy / clip_coords.w;
    // move from [(-1,-1),(1,1)] rectangle to [(0,0),(1,1)]
    vec2 texture_coords = (ndc_coords + 1.0) / 2.0;
    vec2 refract_coords = vec2(texture_coords.x, texture_coords.y);
    vec2 reflect_coords = vec2(texture_coords.x, 1.0 - texture_coords.y);

    vec4 reflection_color = texture(reflection_tex, reflect_coords);
    vec4 refraction_color = texture(refraction_tex, refract_coords);

    final_color = mix(reflection_color, refraction_color, 0.5);    
}