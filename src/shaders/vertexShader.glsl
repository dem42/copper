#version 400 core

in vec3 pos;

out vec3 colour;

void main(void) {
    gl_Position = vec4(pos, 1.0);
    colour = vec3(pos.x + 0.5, 1.0, pos.z + 0.5);
}