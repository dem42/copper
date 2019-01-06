#version 400 core

in vec3 pass_tex_coords;
out vec4 out_Color;

uniform samplerCube cube_map_sampler;

void main(void) {
    out_Color = texture(cube_map_sampler, pass_tex_coords);
}