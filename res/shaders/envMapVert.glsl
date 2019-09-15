#version 400 core

in vec3 position;
in vec2 tex_coord;
in vec3 normal;

out vec2 pass_tex_coord;
out vec3 pass_normal;
out vec3 reflected_camera_pos;
out vec3 refracted_camera_pos;

uniform mat4 model_matrix;
uniform mat4 vp_matrix;
uniform vec3 camera_position;

void main(void) {
    vec4 world_pos = model_matrix * vec4(position, 1.0);
    gl_Position = vp_matrix * world_pos;

    vec3 nnormal = normalize(normal);
    pass_normal = nnormal;

    pass_tex_coord = tex_coord;

    vec3 from_camera = normalize(world_pos.xyz - camera_position);
    reflected_camera_pos = reflect(from_camera, pass_normal);

    float air_to_water_refraction_ratios = 1.0/1.33;
    refracted_camera_pos = refract(from_camera, pass_normal, air_to_water_refraction_ratios);
}