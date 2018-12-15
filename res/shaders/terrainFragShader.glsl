#version 400 core

in vec2 pass_tex_coord;
in vec3 surface_normal;
in vec3 light_direction;

in vec3 to_camera_dir;
in vec3 specular_reflection_dir;
// fog stuff
in float visibility;

// rgba
out vec4 out_Color;

uniform sampler2D background_sampler;
uniform sampler2D r_sampler;
uniform sampler2D g_sampler;
uniform sampler2D b_sampler;
uniform sampler2D blend_map_sampler;
uniform vec3 light_color;

uniform float shine_damper;
uniform float reflectivity;
// fog
uniform vec3 sky_color;

void main(void) {

    // sample untiled (by untiled i mean before coordinates are scaled by 40.0 which exploits REPEAT to tile the texture onto the object)
    vec4 blend_map_col = texture(blend_map_sampler, pass_tex_coord);
     // our bland map pixels are only either some val of r or g or b so this will be <= 1
    float background_coef = 1.0 - (blend_map_col.r + blend_map_col.g + blend_map_col.b);

    // tile the texture by scaling by 40.0 effectively getting 40 * 40 tiles of the same texture onto the model due to REPEAT rather than 1 tile)
    // this is useful if our textures are small and low detail but the object is large
    vec2 tiled_coords = pass_tex_coord * 40.0;
    vec4 background_col = texture(background_sampler, tiled_coords) * background_coef;
    vec4 r_col = texture(r_sampler, tiled_coords) * blend_map_col.r;
    vec4 g_col = texture(g_sampler, tiled_coords) * blend_map_col.g;
    vec4 b_col = texture(b_sampler, tiled_coords) * blend_map_col.b;
    vec4 blended_texture_color = background_col + r_col + g_col + b_col;

    // we have to normalize after interpolation
    vec3 unit_normal = normalize(surface_normal);
    vec3 unit_light = normalize(light_direction);
    
    float dotNormToLight = dot(unit_normal, unit_light);
    float brightness = max(dotNormToLight, 0.2); // clamp to [0.2, 1], the 0.2 means everything is given a little bit of color -> ambient
    vec4 diffuse_color = vec4(brightness * light_color, 1.0); // add alpha of 1
    
    vec3 unit_camera = normalize(to_camera_dir);
    vec3 unit_specular_reflection = normalize(specular_reflection_dir);
    float dotSpecToCamera = dot(unit_camera, unit_specular_reflection);
    float spec_brightness = max(dotSpecToCamera, 0.0);
    vec4 specular_color = vec4(pow(spec_brightness, shine_damper) * reflectivity * light_color, 1.0);
    
    vec4 light_based_out_color = diffuse_color * blended_texture_color + specular_color;
    out_Color = mix(vec4(sky_color, 1.0), light_based_out_color, visibility);
}