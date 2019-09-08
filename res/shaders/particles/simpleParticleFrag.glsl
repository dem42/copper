#version 400 core

in vec3 final_colour;

out vec4 out_Colour;

void main(void){

	out_Colour = vec4(final_colour, 1.0);

}