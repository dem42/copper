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
out vec4 shadow_coords;

uniform mat4 transform;
uniform mat4 projection_matrix;
uniform mat4 view_matrix;
uniform mat4 to_shadowmap_space;
uniform float shadow_distance;

uniform vec3 light_pos[NUM_LIGHTS];
uniform float uses_fake_lighting;

// atlas scaling stuff
uniform float number_of_rows;
uniform vec2 texture_offset;

// fog stuff
const float fog_density = 0.007;
const float fog_gradient = 1.5;

// shadow stuff
const float shadow_transition_distance = 10.0;

// clipping plane for water rendering
uniform vec4 clip_plane;

void main(void) {
    vec4 world_position = transform * vec4(pos, 1.0);
    shadow_coords = to_shadowmap_space * world_position;
    // set what the distance to clipping plane 0 is from this vertex (negative will get culled, positive won't)
    // to compute distance of point from plane we substitute the point (or it's vec4 with w=1) into plane equation -> this is the same as taking dot product
    // because you are basically projecting the vector onto the plane normal and you get the magnitude of this vector in the direction of the normal
    // then you compare this magnitude to the plane's D (distance from origin)
    gl_ClipDistance[0] = dot(world_position, clip_plane);
    
    vec4 eye_space_position = view_matrix * world_position;
    gl_Position = projection_matrix * eye_space_position;
    pass_tex_coord = (tex_coord / number_of_rows) + texture_offset; // rescale original tex_coords down to section of atlas where texture is located
    // tex coords will get linearly interpolated as we pass them to frag shader

    vec3 actual_normal = normal;
    if (uses_fake_lighting > 0.5) {
        actual_normal = vec3(0.0, 0.1, 0.0); // use a fake normal that points up (hack for bad grass model)
    }

    // this i think is correct: you need to transform normals by the transpose of the inverse of the transformation matrix
    mat4 normal_transform = transpose(inverse(transform));
    surface_normal = (normal_transform * vec4(actual_normal, 0.0)).xyz;
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

    float to_shadow_box_edge_dist = distance_to_eye - (shadow_distance - shadow_transition_distance);
    float excess_of_transition = to_shadow_box_edge_dist / shadow_transition_distance;
    shadow_coords.w = 1 - clamp(excess_of_transition, 0, 1);
}