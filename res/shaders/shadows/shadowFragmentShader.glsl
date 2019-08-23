#version 400 core

in vec2 texture_coords;
out vec4 out_Color;

uniform sampler2D model_texture;

void main(void) {

    float alpha = texture(model_texture, texture_coords).a;
    if (alpha < 0.5) {
        discard;
    }

    out_Color = vec4(1.0);
}