#version 450
#extension GL_ARB_explicit_uniform_location : require

layout(location = 0) in vec3 model_position;

layout(location = 0) uniform mat4 world_transform;
layout(location = 1) uniform mat4 view_transform;
layout(location = 2) uniform mat4 perspective_transform;
layout(location = 2) uniform vec3 color;

out vec4 f_color;

vec4 world_to_clip_space(vec4 position) {
    vec4 view_space_position = view_matrix * position;
    vec4 clip_space_position = perspective_matrix * view_space_position;
    return clip_space_position;
}

void main(void) {
    vec4 world_space_position = world_transform * vec4(model_position, 1.0);
    gl_Position = world_to_clip_space(world_space_position);
    f_color = vec4(color, 1.0);
}