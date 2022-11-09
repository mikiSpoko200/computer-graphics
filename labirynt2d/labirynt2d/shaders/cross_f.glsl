#version 330 
#extension GL_ARB_explicit_uniform_location : require
layout(location = 2) uniform vec3 cross_color;
out vec4 color;

void main(void) {
    color = vec4(cross_color, 1.0);
} 