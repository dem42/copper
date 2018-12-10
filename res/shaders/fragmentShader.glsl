#version 400 core

in vec2 pass_tex_coord;
in vec3 surface_normal;
in vec3 light_direction;

in vec3 to_camera_dir;
in vec3 specular_reflection_dir;

// rgba
out vec4 out_Color;

uniform sampler2D texture_sampler;
uniform vec3 light_color;

uniform float shine_damper;
uniform float reflectivity;

void main(void) {
    
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

    out_Color = diffuse_color * texture(texture_sampler, pass_tex_coord) + specular_color; 
}