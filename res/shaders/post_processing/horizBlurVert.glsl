#version 400 core

in vec2 position;
// instead of generating the pixel offsets (offsets needed to know where to sample) on the fragment shader (expensive loop per pixel)
// we generate them on the vertex shader and then for each pixel inside the triangle they are interpolated from offsets at the vertices which is correct
out vec2 blurTextureCoords[11];

uniform float viewport_width;

void main(void) {
    gl_Position = vec4(position, 0.0, 1.0);

    vec2 texture_pos = position * 0.5 + 0.5;
    float pixel_width = 1.0 / viewport_width;
    for (int i=-5; i<=5; i++) {
        blurTextureCoords[i + 5] = texture_pos + vec2(i * pixel_width, 0.0);
    }
}