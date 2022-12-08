#version 450
#extension GL_ARB_explicit_uniform_location : require

layout(location = 0) uniform mat4 perspective_transform;
layout(location = 1) uniform mat4 view_transform;
layout(location = 2) uniform vec3 color;
layout(location = 3) uniform vec3 direction_point;

out vec4 f_color;

vec4 world_to_clip_space(vec4 position) {
    vec4 view_space_position = view_transform * position;
    vec4 clip_space_position = perspective_transform * view_space_position;
    return clip_space_position;
}

void main(void) {
    vec3 model_position;
    if (gl_VertexID == 0) {
        model_position = vec3(0, 0, 0);
    } else {
        model_position = direction_point;
    }
    vec4 world_space_position = vec4(model_position, 1.0);
    gl_Position = world_to_clip_space(world_space_position);
    f_color = vec4(color, 1.0);
}