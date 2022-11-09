#version 330 
#extension GL_ARB_explicit_uniform_location : require
#extension GL_ARB_shading_language_420pack : require

layout(location = 0) in vec2 model_position;
layout(location = 1) in vec3 color;
layout(location = 2) in float angle;

layout(location = 0) uniform int row_tail_count;
layout(location = 1) uniform float aspect_ratio;

out vec4 fcolor;

void main(void) {
    const float x = mod(gl_InstanceID, row_tail_count);
    const float y = float(gl_InstanceID / row_tail_count);
    const float segment_length = 2.0 / float(row_tail_count);
    vec2 offset = {0.0, 0.0};

    if (gl_InstanceID == 0) {
        offset += vec2(2.0, 2.0);
    }

    if (gl_InstanceID == row_tail_count * row_tail_count - 1) {
        offset += vec2(2.0, 2.0);
    }
    const mat2 rotation = {
        { cos(angle), -sin(angle) },
        { sin(angle),  cos(angle) }
    };

    const vec2 top_left_corner = { x * segment_length, y * segment_length };
    const vec2 offset_center = top_left_corner + vec2(segment_length / 2.0, segment_length / 2.0);
    const vec2 center = offset_center - vec2(1.0, 1.0);

    const vec2 transformed_position = rotation * model_position + center + offset;
    
    gl_Position = vec4(transformed_position, 0.0, 1.0);
    fcolor = vec4(color, 1.0);
}