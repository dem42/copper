#version 400 core

const int NUM_LIGHTS = 3;

in vec2 pass_tex_coord;
in vec3 surface_normal;
in vec3 light_direction[NUM_LIGHTS];
// specular lighting stuff
in vec3 to_camera_dir;
in vec3 specular_reflection_dir[NUM_LIGHTS];
// fog stuff
in float visibility;

// rgba
out vec4 out_Color;

uniform sampler2D texture_sampler;
uniform vec3 light_color[NUM_LIGHTS];
// specular lighting
uniform float shine_damper;
uniform float reflectivity;
// fog
uniform vec3 sky_color;

void main(void) {
    
    vec4 texture_color = texture(texture_sampler, pass_tex_coord);
    if (texture_color.a < 0.5) {
        discard; // do not render transparency (hack)
    }
    // we have to normalize after interpolation
    vec3 unit_normal = normalize(surface_normal);
    vec3 unit_camera = normalize(to_camera_dir);

    vec3 total_diffuse = vec3(0.0);
    vec3 total_specular = vec3(0.0);

    for (int i=0; i<NUM_LIGHTS; i++) {
        vec3 unit_light = normalize(light_direction[i]);    
        float dotNormToLight = dot(unit_normal, unit_light);
        float brightness = max(dotNormToLight, 0.0);

        vec3 unit_specular_reflection = normalize(specular_reflection_dir[i]);
        float dotSpecToCamera = dot(unit_camera, unit_specular_reflection);
        float spec_brightness = max(dotSpecToCamera, 0.0);

        total_diffuse += brightness * light_color[i];
        total_specular = pow(spec_brightness, shine_damper) * reflectivity * light_color[i];
    }
    total_diffuse = max(total_diffuse, 0.2); // clamp to 0.2 so nothing totally dark -> ambient light

    vec4 light_based_out_color = vec4(total_diffuse * total_diffuse + total_specular, 1.0);
    out_Color = mix(vec4(sky_color, 1.0), light_based_out_color, visibility);
}