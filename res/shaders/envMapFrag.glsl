#version 400 core

in vec2 pass_tex_coord;
in vec3 pass_normal;
in vec3 reflected_camera_pos;
in vec3 refracted_camera_pos;

out vec4 out_color;

uniform sampler2D in_texture;
uniform samplerCube env_map;

const vec3 light_direction = normalize(vec3(0.3, -1.0, 0.7));
const float ambient = 0.3;

void main(void) {
    float brightness = max(0.0, dot(-light_direction, normalize(pass_normal))) + ambient;

    vec4 obj_color = texture(in_texture, pass_tex_coord) * brightness;
    vec4 reflected_color = texture(env_map, reflected_camera_pos);
    vec4 refracted_color = texture(env_map, refracted_camera_pos);
    vec4 env_color = mix(reflected_color, refracted_color, 0.5);

    out_color = mix(obj_color, env_color, 0.9);
}