#version 400 core

in vec2 pass_tex_coord;

// rgba
out vec4 out_Color;

uniform sampler2D texture_sampler;

void main(void) {    
    out_Color = texture(texture_sampler, pass_tex_coord); // sample texture at those tex coords to get pixel color there
}