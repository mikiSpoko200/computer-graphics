#version 450
#extension GL_ARB_explicit_uniform_location : require

layout(location = 0) in vec3 rotation;

layout(location = 0) uniform mat4 perspective_matrix;
layout(location = 1) uniform mat4 view_matrix;
layout(location = 2) uniform float grid_size;

out vec4 f_color;

// const float grid_size = 10.0;

mat4 rotation_matrix(vec3 axis, float angle)
{
    axis = normalize(axis);
    float s = sin(angle);
    float c = cos(angle);
    float oc = 1.0 - c;

    return mat4(oc * axis.x * axis.x + c,           oc * axis.x * axis.y - axis.z * s,  oc * axis.z * axis.x + axis.y * s,  0.0,
                oc * axis.x * axis.y + axis.z * s,  oc * axis.y * axis.y + c,           oc * axis.y * axis.z - axis.x * s,  0.0,
                oc * axis.z * axis.x - axis.y * s,  oc * axis.y * axis.z + axis.x * s,  oc * axis.z * axis.z + c,           0.0,
                0.0,                                0.0,                                0.0,                                1.0);
}

vec3 rot_axis[3] = vec3[] (
    vec3(1, 0, 0),
    vec3(0, 1, 0),
    vec3(0, 0, 1)
);

vec3 model[3] = vec3[3] (
    vec3(-1, -1,  0),
    vec3( 1, -1,  0),
    vec3( 0,  1,  0)
);

vec3 from_01_to_ndc(vec3 position) {
    return 2 * position - 1;
}

vec3 instance_color(vec3 instance_ndc_position) {
    instance_ndc_position += 1.0;
    instance_ndc_position /= 2.0;
    float r = instance_ndc_position.x * 0.6 + 0.2;
    float g = instance_ndc_position.y * 0.6 + 0.2;
    float b = instance_ndc_position.z * 0.6 + 0.2;
    return vec3(r, g, b);
}

vec3 instance_offset() {
    uint cell_count = uint(grid_size);
    float x = mod(gl_InstanceID, cell_count) / grid_size;
    float y = mod(gl_InstanceID / cell_count, cell_count) / grid_size;
    float z = mod(gl_InstanceID / (cell_count * cell_count), cell_count) / grid_size;

    vec3 grid_center = 2 * vec3(x, y, z) - 1 + 1 / grid_size;
    return grid_center;
}

vec4 world_to_clip_space(vec4 position) {
    vec4 view_space_position = view_matrix * position;
    vec4 clip_space_position = perspective_matrix * view_space_position;
    return clip_space_position;
}

void main(void) {
    float scale = 1 / float(grid_size);
    vec3 position = model[int(mod(gl_VertexID, 3))];
    vec4 rotated = vec4(position, 1.0);;
    for (int i = 0; i < 3; i++) {
        mat4 matrix = rotation_matrix(rot_axis[i], rotation[i]);
        rotated *= matrix;
    }

    vec3 v_index_pos = rotated.xyz * scale / 2 + instance_offset();

    f_color = vec4(instance_color(v_index_pos), 1.0);
    vec4 world_position = vec4(v_index_pos, 1.0);
    gl_Position = world_to_clip_space(world_position);
}