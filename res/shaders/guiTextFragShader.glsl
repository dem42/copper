#version 400 core

in vec2 pass_tex_coord;

out vec4 out_color;

uniform sampler2D df_font_texture;
uniform vec3 color;

const float width = 0.5;
const float edge = 0.1;

// create a solid looking border by choosing a larger width than char width and a small edge just to anti-alias
// const float border_width = 0.7;
// const float border_edge = 0.1;

// create a glow effect by hiding the solid part of the border and making the edge very large 
const float border_width = 0.5;
const float border_edge = 0.4;
const vec3 border_color = vec3(0.0, 1.0, 0.0);
const vec2 shadow_offset = vec2(0.0, 0.0);

// create a drop shadow by choosing a grey border color a fuzzy edge and an offset
// drop shadows may cause outlines of other characters to appear -> to solve make the spacing(padding) larger when you generate the font atlas
// const float border_width = 0.6;
// const float border_edge = 0.3;
// const vec2 shadow_offset = vec2(-0.004, -0.004);
// const vec3 border_color = vec3(0.2, 0.2, 0.2);

void main(void) {
    // this is the distance from the character center -> we use it to draw the character more smoothly using smoothstep function
    float distance = 1.0 - texture(df_font_texture, pass_tex_coord).a;
    float alpha = 1.0 - smoothstep(width, width + edge, distance);

    float distance_border = 1.0 - texture(df_font_texture, pass_tex_coord + shadow_offset).a;
    float alpha_border = 1.0 - smoothstep(border_width, border_width + border_edge, distance_border);

    float overall_alpha = alpha + (1.0 - alpha) * alpha_border;
    vec3 final_color = mix(border_color, color, alpha / overall_alpha);

    out_color = vec4(final_color, overall_alpha);
}