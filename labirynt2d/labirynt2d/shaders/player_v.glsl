#version 330 
#extension GL_ARB_explicit_uniform_location : require
#extension GL_ARB_shading_language_420pack : require

layout(location = 0) in vec2 model_position;
layout(location = 1) uniform vec2 center;
layout(location = 2) uniform mat2 rotation;
layout(location = 3) uniform vec3 color;

out vec3 player_color;

void main(void) {
    vec2 transformed_position = (rotation * model_position + center);
    gl_Position = vec4(transformed_position, 0.0, 1.0);
    player_color = color;
}