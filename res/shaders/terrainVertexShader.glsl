#version 400 core

const int NUM_LIGHTS = 4;

in vec3 pos;
in vec2 tex_coord;
in vec3 normal;

out vec2 pass_tex_coord;
out vec3 surface_normal;
out vec3 light_direction[NUM_LIGHTS];
out vec3 to_camera_dir;
out float visibility;

uniform mat4 transform;
uniform mat4 projection_matrix;
uniform mat4 view_matrix;

uniform vec3 light_pos[NUM_LIGHTS];

// fog stuff
const float fog_density = 0.007;
const float fog_gradient = 1.5;

// clipping plane for water rendering
uniform vec4 clip_plane;

void main(void) {
    vec4 world_position = transform * vec4(pos, 1.0);
    gl_ClipDistance[0] = dot(world_position, clip_plane);

    vec4 eye_space_position = view_matrix * world_position;
    gl_Position = projection_matrix * eye_space_position;
    pass_tex_coord = tex_coord;

    // this i think is incorrect you need to transform by the transpose of the inverse of the transformation matrix
    mat4 normal_transform = transpose(inverse(transform));
    surface_normal = (normal_transform * vec4(normal, 0.0)).xyz;
    for (int i=0; i<NUM_LIGHTS; i++) {
        light_direction[i] = light_pos[i] - world_position.xyz;
    }

    // extract camera position from view matrix
    vec3 camera_position = (inverse(view_matrix) * vec4(0.0, 0.0, 0.0, 1.0)).xyz;
    to_camera_dir = camera_position - world_position.xyz;
    
    // compute visibility
    float distance_to_eye = length(eye_space_position.xyz);
    float fog_vis_coef = exp(-pow(distance_to_eye * fog_density, fog_gradient));
    visibility = clamp(fog_vis_coef, 0.0, 1.0);
}