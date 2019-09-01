#version 400 core

in vec2 position;

out vec2 texture_coords;

void main(void) {
    gl_Position = vec4(position, 0.0, 1.0);
    texture_coords = position * 0.5 + 0.5;
}