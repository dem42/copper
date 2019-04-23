#version 400 core

in vec2 position;
in mat4 model_view_matrix;
in vec4 tex_offsets;
in float blend_factor;

out vec2 tex_coords1;
out vec2 tex_coords2;
out float blend;

uniform mat4 projection_matrix;
uniform float number_of_rows;

void main(void) {
    // the quad we use for particles has extent (-0.5, 0.5) -> (0.5, -0.5)
    // we want it to be mapped to the entire particle texture so to (0, 0) -> (1, 1) which should be the texture coords
    vec2 tex_coords = position + vec2(0.5, 0.5);
    tex_coords.y = 1.0 - tex_coords.y;
    // scale down by how many images there are in atlas
    tex_coords /= number_of_rows;
    tex_coords1 = tex_coords + tex_offsets.xy;
    tex_coords2 = tex_coords + tex_offsets.zw;
    blend = blend_factor;

    vec4 world_pos = projection_matrix * model_view_matrix * vec4(position, 0, 1);
    gl_Position = world_pos;
}