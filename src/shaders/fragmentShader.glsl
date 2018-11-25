#version 400 core

in vec3 colour;

// rgba
out vec4 out_Color;

void main(void) {
    // fragment shader has one output which is pixel color
    out_Color = vec4(colour, 1.0);
}