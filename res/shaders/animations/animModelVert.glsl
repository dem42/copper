#version 400 core

const int MAX_JOINTS = 50;
const int MAX_WEIGHTS = 4;

in vec3 in_position;
in vec2 in_tex_coords;
in vec3 in_normal;
in ivec4 in_joint_indicies;
in vec4 in_joint_weights;

out vec2 pass_tex_coords;
out vec3 pass_normal;

uniform mat4 joint_transforms[MAX_JOINTS];
uniform mat4 projection_view_model;

void main(void) {
    vec4 total_pos = vec4(0);
    vec4 total_normal = vec4(0);

    vec4 pos4 = vec4(in_position, 1.0);
    vec4 norm4 = vec4(in_normal, 0.0);

    for (int i=0; i<MAX_WEIGHTS; i++) {
        vec4 pos_c = joint_transforms[in_joint_indicies[i]] * pos4;
        vec4 norm_c = joint_transforms[in_joint_indicies[i]] * norm4;
        total_pos += pos_c * in_joint_weights[i];
        total_normal += norm_c * in_joint_weights[i];
    }    
    gl_Position = projection_view_model * total_pos;
    pass_normal = total_normal.xyz;

    // gl_Position = projection_view_model * pos4;
    // pass_normal = norm4.xyz;
    pass_tex_coords = in_tex_coords;
}