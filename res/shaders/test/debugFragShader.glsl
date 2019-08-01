#version 400 core

in vec4 color;
out vec4 out_Color;

void main(void) {
    out_Color = color;
    out_Color.a = 0.2;
    //out_Color = vec4(1, 0, 1, 0.2);
}