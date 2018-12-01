#version 400 core

in vec3 pos;
in vec2 tex_coord;

out vec2 pass_tex_coord;

void main(void) {
    gl_Position = vec4(pos, 1.0);
    pass_tex_coord = tex_coord; // get linearly interpolated as we pass them to frag shader
}