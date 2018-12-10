#version 400 core

in vec3 pos;
in vec2 tex_coord;
in vec3 normal;

out vec2 pass_tex_coord;
out vec3 surface_normal;
out vec3 light_direction;
out vec3 to_camera_dir;
out vec3 specular_reflection_dir;

uniform mat4 transform;
uniform mat4 projection_matrix;
uniform mat4 view_matrix;

uniform vec3 light_pos;
uniform float uses_fake_lighting;

void main(void) {
    vec4 world_position = transform * vec4(pos, 1.0);
    gl_Position = projection_matrix * view_matrix * world_position;
    pass_tex_coord = tex_coord; // get linearly interpolated as we pass them to frag shader

    vec3 actual_normal = normal;
    if (uses_fake_lighting > 0.5) {
        actual_normal = vec3(0.0, 0.1, 0.0); // use a fake normal that points up (hack for bad grass model)
    }

    // this i think is correct: you need to transform normals by the transpose of the inverse of the transformation matrix
    mat4 normal_transform = transpose(inverse(transform));
    surface_normal = (normal_transform * vec4(actual_normal, 0.0)).xyz;
    light_direction = light_pos - world_position.xyz;

    // extract camera position from view matrix
    vec3 camera_position = (inverse(view_matrix) * vec4(0.0, 0.0, 0.0, 1.0)).xyz;
    to_camera_dir = camera_position - world_position.xyz;
    specular_reflection_dir = reflect(-light_direction, surface_normal);
}