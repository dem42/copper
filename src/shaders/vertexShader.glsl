#version 400 core

in vec3 pos;
in vec2 tex_coord;
in vec3 normal;

out vec2 pass_tex_coord;
out vec3 surface_normal;
out vec3 light_direction;

uniform mat4 transform;
uniform mat4 projection_matrix;
uniform mat4 view_matrix;

uniform vec3 light_pos;

void main(void) {
    vec4 world_position = transform * vec4(pos, 1.0);
    gl_Position = projection_matrix * view_matrix * world_position;
    pass_tex_coord = tex_coord; // get linearly interpolated as we pass them to frag shader

    // this i think is incorrect you need to transform by the transpose of the inverse of the transformation matrix
    mat4 normal_transform = transpose(inverse(transform));
    surface_normal = (normal_transform * vec4(normal, 0.0)).xyz;
    light_direction = light_pos - world_position.xyz;
}