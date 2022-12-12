#version 450

in vec4 f_color;
out vec4 pixel_color;

void main(void) {
   pixel_color = f_color;
}