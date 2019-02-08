#version 400 core

in vec3 pass_tex_coords;
out vec4 out_Color;

uniform vec3 fog_color;
uniform samplerCube cube_map_sampler1;
uniform samplerCube cube_map_sampler2;
uniform float blend_factor;

// lower limit is up to where the skybox should have the color of the fog
const float lower_limit = 0.0;
// uper limit is from where we dont mix with fog color at all
const float upper_limit = 30.0;

const bool uses_cell_shading = true;
const float brightness_levels = 10.0;

void adjust_brightness(inout vec4 skybox_color) {
    if (!uses_cell_shading) {
        return;
    } else {
        float luma = 0.375 * skybox_color.x + 0.5 * skybox_color.y + 0.125 * skybox_color.z;

        // this assumes that the brightness is in [0,1] interval and so we use it to interpolate
        float celled_luma = floor(luma * brightness_levels) / brightness_levels;
        skybox_color = vec4(fog_color, 1.0) * celled_luma;
    }
}

void main(void) {
    vec4 day_color = texture(cube_map_sampler1, pass_tex_coords);
    vec4 night_color = texture(cube_map_sampler2, pass_tex_coords);
    vec4 final_color = mix(day_color, night_color, blend_factor);

    // tex coord y is pixel y coord in case of skybox
    float factor = (pass_tex_coords.y - lower_limit) / (upper_limit - lower_limit);
    factor = clamp(factor, 0.0, 1.0);

    out_Color = mix(vec4(fog_color, 1.0), final_color, factor);
}