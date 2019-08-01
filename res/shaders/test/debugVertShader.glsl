#version 400 core

in vec3 pos;
out vec4 color;

const vec4 colors[8] = vec4[8](vec4(1,0,0,1), vec4(1,1,1,1), vec4(1,1,1,1), vec4(1,1,1,1),
vec4(0,0,0,1), vec4(0,0,0,1), vec4(0,0,0,1), vec4(0,0,0,1));
uniform mat4 mvp_matrix;

void main(void) {
    gl_Position = mvp_matrix * vec4(pos, 1);

    color = colors[gl_VertexID];
}