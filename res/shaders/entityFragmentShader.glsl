#version 400 core

const int NUM_LIGHTS = 4;

in vec2 pass_tex_coord;
in vec3 surface_normal;
in vec3 light_direction[NUM_LIGHTS];
// specular lighting stuff
in vec3 to_camera_dir;
// fog stuff
in float visibility;
in vec4 shadow_coords;

// rgba
out vec4 out_Color;

uniform sampler2D texture_sampler;
uniform sampler2D shadow_map;
uniform sampler2D extra_info_map;

uniform vec3 light_color[NUM_LIGHTS];
// specular lighting
uniform float shine_damper;
uniform float reflectivity;
// fog
uniform vec3 sky_color;
// point light attenuation
uniform vec3 attenuation[NUM_LIGHTS];
// for turning off/on extra info
uniform float has_extra_info;

const bool uses_cell_shading = false;
const float brightness_levels = 3.0;

// how many pixels to sample on each side of center pixel (so 2 means 3x3 box) 
const int pcf_count = 2;
// texture pixels we will be sampling
const float texel_count = (pcf_count*2.0 + 1.0)*(pcf_count*2.0 + 1.0);
uniform float shadow_map_size;

void adjust_brightness(inout float diffuse_brightness, inout float specular_brightness) {
    if (!uses_cell_shading) {
        return;
    } else {
        // this assumes that the brightness is in [0,1] interval and so we use it to interpolate
        diffuse_brightness = floor(diffuse_brightness * brightness_levels) / brightness_levels;
        specular_brightness = floor(specular_brightness * brightness_levels) / brightness_levels;                
    }
}

void main(void) {
    // size of a pixel in texture coords space
    float texel_size = 1.0 / shadow_map_size;
    float total_in_shadow = 0.0;

    for (int x=-pcf_count; x<=pcf_count; x++) {
        for (int y=-pcf_count; y <= pcf_count; y++) {
            // compare depth with shadowmap depth to figure out if this piece of terrain is in shadow or not (absence of light due to something blocking it)
            float obj_depth_nearest_light = texture(shadow_map, shadow_coords.xy + vec2(x, y) * texel_size).r;
            // add slight offset to prevent shadow acne
            // note that unlike in terrain shader here we use a very aggressive bias
            // this due to self-shadow casting of complex objects causes a lot of acne
            // a better approach would be to calculate more precise near/far planes like described in the msdn shadows article
            total_in_shadow += step(obj_depth_nearest_light + 0.01, shadow_coords.z);
        }
    }
    total_in_shadow /= texel_count;     
    float light_factor = 1.0 - total_in_shadow*shadow_coords.w;    
    
    vec4 texture_color = texture(texture_sampler, pass_tex_coord);
    if (texture_color.a < 0.5) {
        discard; // do not render transparency (hack)
    }    
    float shininess_fac = 1.0;
    float glow_fac = 0.0;
    if (has_extra_info > 0.5) {
        vec4 extra_info = texture(extra_info_map, pass_tex_coord);
        shininess_fac = extra_info.r;
        glow_fac = extra_info.g;
    }

    // we have to normalize after interpolation
    vec3 unit_normal = normalize(surface_normal);
    vec3 unit_camera = normalize(to_camera_dir);

    vec3 total_diffuse = vec3(0.0);
    vec3 total_specular = vec3(0.0);

    for (int i=0; i<NUM_LIGHTS; i++) {
        float distance_to_light_point = length(light_direction[i]);
        float attenuation_factor = attenuation[i].x + attenuation[i].y * distance_to_light_point + attenuation[i].z * distance_to_light_point * distance_to_light_point;
        
        vec3 unit_light = normalize(light_direction[i]);    
        float dotNormToLight = dot(unit_normal, unit_light);
        float brightness = max(dotNormToLight, 0.0);

        vec3 specular_reflection_dir = reflect(-light_direction[i], unit_normal);
        vec3 unit_specular_reflection = normalize(specular_reflection_dir);

        float dotSpecToCamera = dot(unit_camera, unit_specular_reflection);
        float spec_brightness = max(dotSpecToCamera, 0.0);

        adjust_brightness(brightness, spec_brightness);

        total_diffuse += (brightness * light_color[i]) / attenuation_factor;
        total_specular = (pow(spec_brightness, shine_damper) * reflectivity * light_color[i]) / attenuation_factor;
    }
    total_diffuse = max(total_diffuse * light_factor, 0.2); // clamp to 0.2 so nothing totally dark -> ambient light

    // apply extra info factors
    // adjust shininess (lowering it) based on specular map
    total_specular *= shininess_fac;
    // when glow_fac > 0.5 this will subtract old total_diffuse and add vec3(1) making the pixel glow since very bright    
    total_diffuse += step(0.5, glow_fac) * (vec3(1) - total_diffuse);

    vec4 light_based_out_color = vec4(total_diffuse, 1.0) * texture_color + vec4(total_specular, 1.0);
    out_Color = mix(vec4(sky_color, 1.0), light_based_out_color, visibility);    
}