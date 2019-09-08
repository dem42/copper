#version 400 core

in vec3 pos;

out vec3 pass_colour;

uniform mat4 vp_matrix;

void main(void){

	gl_PointSize = 2;
	//gl_Position = mix(vp_matrix * vec4(pos, 1.0), vec4(-4.6, 8.17, 49.76, 49.95), 0.999);
	gl_Position = vp_matrix * vec4(pos, 1.0);
	
	pass_colour = vec3(1.0, 0.0, 0.0);
}