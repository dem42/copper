#version 400 core

const int NUM_LIGHTS = 4;

in vec3 pos;
in vec2 tex_coord;
in vec3 normal;
in vec4 tangents;

out vec2 pass_tex_coord;
out vec3 light_direction_tgs[NUM_LIGHTS];
out vec3 to_camera_dir_tgs;
out float visibility;

uniform mat4 transform;
uniform mat4 projection_matrix;
uniform mat4 view_matrix;

uniform vec3 light_pos[NUM_LIGHTS];
uniform float uses_fake_lighting;

// atlas scaling stuff
uniform float number_of_rows;
uniform vec2 texture_offset;

// fog stuff
const float fog_density = 0.007;
const float fog_gradient = 1.5;

// clipping plane for water rendering
uniform vec4 clip_plane;

void main(void) {
    vec4 world_position = transform * vec4(pos, 1.0);
    // set what the distance to clipping plane 0 is from this vertex (negative will get culled, positive won't)
    // to compute distance of point from plane we substitute the point (or it's vec4 with w=1) into plane equation -> this is the same as taking dot product
    // because you are basically projecting the vector onto the plane normal and you get the magnitude of this vector in the direction of the normal
    // then you compare this magnitude to the plane's D (distance from origin)
    gl_ClipDistance[0] = dot(world_position, clip_plane);
    
    mat4 local_to_eye = view_matrix * transform;

    vec4 eye_space_position = view_matrix * world_position;
    gl_Position = projection_matrix * eye_space_position;
    pass_tex_coord = (tex_coord / number_of_rows) + texture_offset; // rescale original tex_coords down to section of atlas where texture is located
    // tex coords will get linearly interpolated as we pass them to frag shader

    vec3 actual_normal = normal;
    if (uses_fake_lighting > 0.5) {
        actual_normal = vec3(0.0, 0.1, 0.0); // use a fake normal that points up (hack for bad grass model)
    }
    
    vec3 tang = tangents.xyz;    

    vec3 tang_eye = (local_to_eye * vec4(tang, 0.0)).xyz; 
    // this i think is correct: you need to transform normals by the transpose of the inverse of the transformation matrix
    // however if you dont need translation then you transform we if 3x3 matrix really an
    // if there is no skew then the transform matrix is orthonormal (rotation + scaling) then the inverse is the transpose and 
    // so transpose transpose is identity 
    mat4 normal_transform = local_to_eye;
    vec3 surface_normal_eye = (normal_transform * vec4(actual_normal, 0.0)).xyz;    
    //vec3 bitang_eye = cross(tang_eye, surface_normal_eye) * tangents.w; we should be multiplying by handedness but it causes oddness
    vec3 bitang_eye = cross(tang_eye, surface_normal_eye);
    
    tang_eye = normalize(tang_eye);
    bitang_eye = normalize(bitang_eye);
    surface_normal_eye = normalize(surface_normal_eye);
    // if you want to see non bump mapped transform surface_normal_eye to tangent space and pass it to fragment shader

    // NOTE!!! glsl shader mat3 constructor takes column vectors!!
    // so this is the matrix transposed (and since it is orthonormal that means inverted)
    mat3 eye_to_tangent_space = mat3(
        tang_eye.x, bitang_eye.x, surface_normal_eye.x,
        tang_eye.y, bitang_eye.y, surface_normal_eye.y,
        tang_eye.z, bitang_eye.z, surface_normal_eye.z
    );

    for (int i=0; i<NUM_LIGHTS; i++) {
        light_direction_tgs[i] = eye_to_tangent_space * (view_matrix * (vec4(light_pos[i], 1.0) - world_position)).xyz;  
    }    
    to_camera_dir_tgs = eye_to_tangent_space * (-eye_space_position.xyz);
        
    // compute visibility
    float distance_to_eye = length(eye_space_position.xyz);
    float fog_vis_coef = exp(-pow(distance_to_eye * fog_density, fog_gradient));
    visibility = clamp(fog_vis_coef, 0.0, 1.0);
}