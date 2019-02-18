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

uniform float wave_factor;
uniform vec3 light_color[LIGHT_NUM];
uniform vec3 attenuation[LIGHT_NUM];

const float wave_strength = 0.02;
const float water_reflectivity = 1.5;

const float shine_damper = 20.0;
const float shine_reflectivity = 0.8;

void main() {
    vec2 ndc_coords = clip_coords.xy / clip_coords.w;
    // move from [(-1,-1),(1,1)] rectangle to [(0,0),(1,1)]
    vec2 texture_coords = (ndc_coords + 1.0) / 2.0;
    vec2 refract_coords = vec2(texture_coords.x, texture_coords.y);
    vec2 reflect_coords = vec2(texture_coords.x, 1 - texture_coords.y);

    // this seems like a fancy way to distort the water
    vec2 distorted_tex_coords = texture(dudv_map, vec2(tex_coords.x + wave_factor, tex_coords.y)).rg * 0.1;
    distorted_tex_coords = tex_coords + vec2(distorted_tex_coords.x, distorted_tex_coords.y + wave_factor);
    vec2 total_distortion = (texture(dudv_map, distorted_tex_coords).rg * 2.0 - 1.0) * wave_strength;
    
    reflect_coords += total_distortion;
    reflect_coords = clamp(reflect_coords, 0.001, 0.999);
    refract_coords += total_distortion;
    refract_coords = clamp(refract_coords, 0.001, 0.999);

    vec4 reflection_color = texture(reflection_tex, reflect_coords);
    vec4 refraction_color = texture(refraction_tex, refract_coords);

    vec3 normalize_to_cam = normalize(to_camera_vec);
    vec3 water_normal = vec3(0.0, 1.0, 0.0);
    // 1 if to camera in same direction as water normal, 0 if perpendicular
    float refraction_factor = dot(normalize_to_cam, water_normal);
    refraction_factor = pow(refraction_factor, water_reflectivity);

    vec4 normal_color = texture(normal_map, distorted_tex_coords);
    // we want negative values in x and y in the normals
    vec3 normal = vec3(normal_color.r * 2.0 - 1.0, normal_color.b, normal_color.g * 2.0 - 1.0);
    normal = normalize(normal);

    vec3 total_specular = vec3(0.0);
    for (int i = 0; i < LIGHT_NUM; ++i) {
        float dist = length(from_light[i]);
        float attenuation_factor = attenuation[i].x + attenuation[i].y * dist + attenuation[i].z * dist * dist;

        vec3 reflected = reflect(from_light[i], normal);
        reflected = normalize(reflected);
        float spec_factor = max(dot(reflected, normalize_to_cam), 0.0);
        total_specular += (pow(spec_factor, shine_damper) * shine_reflectivity * light_color[i]) / attenuation_factor;
    }
    
    reflection_color += vec4(total_specular, 0.0);
    final_color = mix(reflection_color, refraction_color, refraction_factor);
    // mix with a bit of blue/gree
    final_color = mix(final_color, vec4(0.0, 0.3, 0.5, 1.0), 0.2);    
}