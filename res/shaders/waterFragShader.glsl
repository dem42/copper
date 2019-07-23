#version 400 core

const int LIGHT_NUM = 4;

in vec4 clip_coords;
in vec2 tex_coords;
in vec3 to_camera_vec;
in vec3 from_light[LIGHT_NUM];

out vec4 final_color;

uniform sampler2D reflection_tex;
uniform sampler2D refraction_tex;
uniform sampler2D dudv_map;
uniform sampler2D normal_map;
uniform sampler2D depth_map;

uniform float wave_factor;
uniform vec3 light_color[LIGHT_NUM];
uniform vec3 attenuation[LIGHT_NUM];

// these are the coefficients from the perspective transform matrix
// we use them to get the real depth (real z) from the ndc coord z [-1,1] range
// the depth buffer in fact has the value in the range [0,1]
uniform float depth_calc_A;
uniform float depth_calc_B;

// fog
uniform vec3 sky_color;

const float wave_strength = 0.04;
const float water_reflectivity = 1.5;

const float shine_damper = 20.0;
const float shine_reflectivity = 0.5;

const float fog_density = 0.007;
const float fog_gradient = 1.5;

void main() {
    vec2 ndc_coords = clip_coords.xy / clip_coords.w;
    // move from [(-1,-1),(1,1)] rectangle to [(0,0),(1,1)]
    vec2 texture_coords = (ndc_coords + 1.0) / 2.0;
    vec2 refract_coords = vec2(texture_coords.x, texture_coords.y);
    vec2 reflect_coords = vec2(texture_coords.x, 1 - texture_coords.y);

    float bottom_to_camera = texture(depth_map, texture_coords).x;
    // use depth buffer reversal formula 
    float bottom_to_camera_real_z = -depth_calc_B / (depth_calc_A + 2.0*bottom_to_camera - 1.0);
    bottom_to_camera_real_z = -bottom_to_camera_real_z; // we want depths to be positive unlike z

    float water_surface_depth = gl_FragCoord.z; // find frags depth buffer z ( i think to be able to do this we have to create a renderbuffer attachment like we did in framebuffers -> test)
    float water_surface_depth_real_z = -(-depth_calc_B / (depth_calc_A + 2.0*water_surface_depth - 1.0)); // the minus is from wanting positive like above
    float water_depth = bottom_to_camera_real_z - water_surface_depth_real_z;
    // alpha blending linearly until distance of 5 into water depth -> after that opaque
    float water_blend_factor = clamp(water_depth / 2.0, 0.0, 1.0);
    // distortion factor
    float water_depth_anti_distort_factor = clamp(water_depth / 20.0, 0.0, 1.0);

    // this seems like a fancy way to distort the water
    vec2 distorted_tex_coords = texture(dudv_map, vec2(tex_coords.x + wave_factor, tex_coords.y)).rg * 0.1;
    distorted_tex_coords = tex_coords + vec2(distorted_tex_coords.x, distorted_tex_coords.y + wave_factor);
    vec2 total_distortion = (texture(dudv_map, distorted_tex_coords).rg * 2.0 - 1.0) * wave_strength * water_depth_anti_distort_factor;
    
    reflect_coords += total_distortion;
    reflect_coords = clamp(reflect_coords, 0.001, 0.999);
    refract_coords += total_distortion;
    refract_coords = clamp(refract_coords, 0.001, 0.999);

    vec4 reflection_color = texture(reflection_tex, reflect_coords);
    vec4 refraction_color = texture(refraction_tex, refract_coords);
    
    vec4 normal_color = texture(normal_map, distorted_tex_coords);
    // we want negative values in x and y in the normals
    // we also want the normals to be all sort of pointing up not go in all directions -> stead water
    const float water_flattness = 3.0;
    vec3 normal = vec3(normal_color.r * 2.0 - 1.0, normal_color.b * water_flattness, normal_color.g * 2.0 - 1.0);
    normal = normalize(normal);

    vec3 normalize_to_cam = normalize(to_camera_vec);    
    // 1 if to camera in same direction as water normal, 0 if perpendicular
    float refraction_factor = dot(normalize_to_cam, normal);
    refraction_factor = pow(refraction_factor, water_reflectivity);
    refraction_factor = clamp(refraction_factor, 0.0, 1.0);

    vec3 total_specular = vec3(0.0);
    for (int i = 0; i < LIGHT_NUM; ++i) {
        float dist = length(from_light[i]);
        float attenuation_factor = attenuation[i].x + attenuation[i].y * dist + attenuation[i].z * dist * dist;

        vec3 reflected = reflect(from_light[i], normal);
        reflected = normalize(reflected);
        float spec_factor = max(dot(reflected, normalize_to_cam), 0.0);
        total_specular += (pow(spec_factor, shine_damper) * shine_reflectivity * light_color[i]) / attenuation_factor;
    }
        
    // compute visibility    
    float fog_vis_coef = exp(-pow(water_surface_depth_real_z * fog_density, fog_gradient));
    float visibility = clamp(fog_vis_coef, 0.0, 1.0);

    final_color = mix(reflection_color, refraction_color, refraction_factor);
    // mix with a bit of blue/gree
    final_color = mix(final_color, vec4(0.0, 0.3, 0.5, 1.0), 0.2) + vec4(total_specular * water_blend_factor, 0.0);;    
    final_color = mix(vec4(sky_color, 1.0), final_color, visibility);
    final_color.a = water_blend_factor;
}