#version 330 
#extension GL_ARB_explicit_uniform_location : require

in vec4 fcolor;
out vec4 color;

void main(void) {
    color = fcolor;
} 