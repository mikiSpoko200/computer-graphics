#version 330 
#extension GL_ARB_explicit_uniform_location : require

layout(location = 0) in vec2 position;
layout(location = 1) in vec3 color;

out vec4 f_color;

void main(void) {
    gl_Position = vec4(position, 0.0, 1.0);
    f_color = vec4(color, 1.0);
} 