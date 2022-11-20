#version 330 
#extension GL_ARB_explicit_uniform_location : require

in vec4 f_color;
out vec4 pixel_color;

void main(void) {
   pixel_color = f_color;
} 