#version 400 core

in vec3 position;

out vec3 pass_tex_coords;

uniform mat4 projection_matrix;
uniform mat4 view_matrix;

// clipping plane for water rendering
uniform vec4 clip_plane;

void main(void) {
    vec4 world_pos = vec4(position, 1.0);
    gl_ClipDistance[0] = dot(world_pos, clip_plane);

    gl_Position = projection_matrix * view_matrix * world_pos;
    pass_tex_coords = position;
}