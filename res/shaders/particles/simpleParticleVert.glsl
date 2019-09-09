#version 400 core

in vec3 pos;

out vec3 pass_colour;

void main(void){
	gl_Position = vec4(pos, 1.0);
	
	pass_colour = vec3(1.0);
}