#version 400 core

// type of input primitives
layout (points) in;
// type of shader output primitives
layout (triangle_strip) out;
// this says how many vertices we are allowed to output across all primitives that we spit out
layout (max_vertices = 24) out;

// will only have one value since our input primitive is points so only one vertex
// per primitive and thus only one color input
in vec3 pass_colour[];

out vec3 final_colour;

uniform mat4 projectionViewMatrix;

const float size = 0.1;
// direction from light source to our point
const vec3 light_direction = normalize(vec3(0.4, -1.0, 0.8));

void createVertex(vec3 offset, float brightness) {
    vec4 actualOffset = vec4(offset * size, 0.0);
    // access built in gl_Position that comes from vertex shader
    vec4 worldPosition = gl_in[0].gl_Position + actualOffset;
    gl_Position = projectionViewMatrix * worldPosition;
    final_colour = pass_colour[0] * brightness;
    EmitVertex();
}

float computeBrightness(vec3 faceNormal) {
	float diffuse_brightness = dot(-light_direction, faceNormal);
	float diffuse_and_ambient = max(diffuse_brightness, 0.4);
	return diffuse_and_ambient;
}

void main(void) {
    // create the triangle strip -> note that sequence creates triangles
    // where the last point creates a triangle with the previous two points
	float brightness = computeBrightness(vec3(0.0, 0.0, 1.0));
    createVertex(vec3(-1.0, 1.0, 1.0), brightness);
	createVertex(vec3(-1.0, -1.0, 1.0), brightness);
	createVertex(vec3(1.0, 1.0, 1.0), brightness);
	createVertex(vec3(1.0, -1.0, 1.0), brightness);
	
	EndPrimitive();
	
	brightness = computeBrightness(vec3(1.0, 0.0, 0.0));
	createVertex(vec3(1.0, 1.0, 1.0), brightness);
	createVertex(vec3(1.0, -1.0, 1.0), brightness);
	createVertex(vec3(1.0, 1.0, -1.0), brightness);
	createVertex(vec3(1.0, -1.0, -1.0), brightness);
	
	EndPrimitive();
	
	brightness = computeBrightness(vec3(0.0, 0.0, -1.0));
	createVertex(vec3(1.0, 1.0, -1.0), brightness);
	createVertex(vec3(1.0, -1.0, -1.0), brightness);
	createVertex(vec3(-1.0, 1.0, -1.0), brightness);
	createVertex(vec3(-1.0, -1.0, -1.0), brightness);
	
	EndPrimitive();
	
	brightness = computeBrightness(vec3(-1.0, 0.0, 0.0));
	createVertex(vec3(-1.0, 1.0, -1.0), brightness);
	createVertex(vec3(-1.0, -1.0, -1.0), brightness);
	createVertex(vec3(-1.0, 1.0, 1.0), brightness);
	createVertex(vec3(-1.0, -1.0, 1.0), brightness);
	
	EndPrimitive();
	
	brightness = computeBrightness(vec3(0.0, 1.0, 0.0));
	createVertex(vec3(1.0, 1.0, 1.0), brightness);
	createVertex(vec3(1.0, 1.0, -1.0), brightness);
	createVertex(vec3(-1.0, 1.0, 1.0), brightness);
	createVertex(vec3(-1.0, 1.0, -1.0), brightness);
	
	EndPrimitive();
	
	brightness = computeBrightness(vec3(0.0, -1.0, 0.0));
	createVertex(vec3(-1.0, -1.0, 1.0), brightness);	
	createVertex(vec3(-1.0, -1.0, -1.0), brightness);
	createVertex(vec3(1.0, -1.0, 1.0), brightness);
	createVertex(vec3(1.0, -1.0, -1.0), brightness);
	
	EndPrimitive();
}