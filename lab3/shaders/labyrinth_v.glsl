#version 450
#extension GL_ARB_explicit_uniform_location : require

layout(location = 0) in vec3 position;

layout(location = 0) uniform mat4 perspective_matrix;
layout(location = 1) uniform mat4 view_matrix;

out vec4 f_color;

vec3 from_01_to_ndc(vec3 position) {
    return 2 * position - 1;
}

vec3 instance_color(vec3 instance_ndc_position) {
    instance_ndc_position += 1.0;
    instance_ndc_position /= 2.0;
    float r = instance_ndc_position.x * 0.6 + 0.2;
    float g = instance_ndc_position.y * 0.6 + 0.2;
    float b = instance_ndc_position.z * 0.6 + 0.2;
    return vec3(r, g, b);
}

vec4 world_to_clip_space(vec4 position) {
    vec4 view_space_position = view_matrix * position;
    vec4 clip_space_position = perspective_matrix * view_space_position;
    return clip_space_position;
}

void main(void) {
    f_color = vec4(instance_color(position), 1.0);
    vec4 world_position = vec4(position, 1.0);
    gl_Position = world_to_clip_space(world_position);
}