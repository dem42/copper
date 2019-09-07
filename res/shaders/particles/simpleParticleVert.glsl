#version 400 core

in vec3 pos;

out vec3 pass_colour;

uniform mat4 vp_matrix;

void main(void){

	gl_PointSize = 2;
	gl_Position = vp_matrix * vec4(pos, 1.0);
	
	pass_colour = vec3(1.0);
}