#version 400 core

in vec3 pos;

uniform mat4 mvp_matrix;

void main(void) {
    gl_Position = mvp_matrix * vec4(pos, 1.0);
}