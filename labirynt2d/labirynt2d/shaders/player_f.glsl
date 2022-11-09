#version 330 
#extension GL_ARB_explicit_uniform_location : require

in vec3 player_color;
out vec4 color;

void main(void) {
    color = vec4(player_color, 1.0);
}