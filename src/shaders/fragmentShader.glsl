#version 400 core

in vec2 pass_tex_coord;
in vec3 surface_normal;
in vec3 light_direction;

// rgba
out vec4 out_Color;

uniform sampler2D texture_sampler;
uniform vec3 light_color;

void main(void) {
    
    // we have to normalize after interpolation
    vec3 unit_normal = normalize(surface_normal);
    vec3 unit_light = normalize(light_direction);
    
    float dotNormToLight = dot(unit_normal, unit_light);
    float brightness = max(dotNormToLight, 0.0); // clamp to [0, 1]
    vec4 diffuse_color = vec4(brightness * light_color, 1.0); // add alpha of 1

    out_Color = diffuse_color * texture(texture_sampler, pass_tex_coord); // sample texture at those tex coords to get pixel color there
}