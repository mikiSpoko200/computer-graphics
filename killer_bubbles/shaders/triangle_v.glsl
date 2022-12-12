#version 450
#extension GL_ARB_explicit_uniform_location : require

layout(location = 0) in vec3 position;

layout(location = 0) uniform mat4 perspective_matrix;
layout(location = 1) uniform mat4 view_matrix;
layout(location = 2) uniform vec3 color;

out vec4 f_color;

vec4 to_clip_space(vec4 position) {
    vec4 view_space_position = view_matrix * position;
    vec4 clip_space_position = perspective_matrix * view_space_position;
    return clip_space_position;
}

void main(void) {
    f_color = vec4(color, 1.0);
    vec4 world_position = vec4(position + vec3(3, 0, 0), 1.0);
    gl_Position = to_clip_space(world_position);
}