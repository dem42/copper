#version 400 core

const int LIGHT_NUM = 4;

in vec3 position;

out vec4 clip_coords;
out vec2 tex_coords;
out vec3 to_camera_vec;
out vec3 from_light[LIGHT_NUM];

uniform mat4 projection_matrix;
uniform mat4 view_matrix;
uniform mat4 transform_matrix;
uniform vec3 camera_world_pos;
uniform vec3 light_pos[LIGHT_NUM];

const float tiling = 6.0;

void main() {
    vec4 world_pos = transform_matrix * vec4(position, 1.0);
    clip_coords = projection_matrix * view_matrix * world_pos;
    gl_Position = clip_coords;

    tex_coords = (position.xz / 2.0 + 0.5) * tiling;

    to_camera_vec = camera_world_pos - world_pos.xyz;
    for (int i=0; i < LIGHT_NUM; ++i) {
        from_light[i] = world_pos.xyz - light_pos[i];
    }
}