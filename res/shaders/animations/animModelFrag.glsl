#version 400 core

// diffuse, ambient factors
const vec2 light_bias = (0.7, 0.6);

in vec2 pass_tex_coords;
in vec3 pass_normal;

out vec4 out_color;

uniform sampler2D diffuse_map;
uniform vec3 light_direction;

void main(void) {
    vec4 color = texture(diffuse_map, pass_tex_coords);
    vec3 normal = normalize(pass_normal);

    float brightness = max(0.0, dot(-light_direction, normal)) * light_bias.x + light_bias.y;
    out_color = color * brightness;
}