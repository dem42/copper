#version 400 core

in vec3 pos;
in vec2 tex_coord;

out vec2 pass_tex_coord;

uniform mat4 transform;
uniform mat4 projection_matrix;
uniform mat4 view_matrix;

void main(void) {
    gl_Position = projection_matrix * view_matrix * transform * vec4(pos, 1.0);
    pass_tex_coord = tex_coord; // get linearly interpolated as we pass them to frag shader
}