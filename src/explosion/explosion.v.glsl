#version 330 core

layout(std140) uniform;

out vec2 mapping;

uniform Locals {
  mat4 transform;
  float radius;
};

void main() {
    vec2 offset;
    switch(gl_VertexID)
    {
    case 0:
        //Bottom-left
        mapping = vec2(-1.0, -1.0);
        offset = vec2(-radius, -radius);
        break;
    case 1:
        //Top-left
        mapping = vec2(-1.0, 1.0);
        offset = vec2(-radius, radius);
        break;
    case 2:
        //Bottom-right
        mapping = vec2(1.0, -1.0);
        offset = vec2(radius, -radius);
        break;
    case 3:
        //Top-right
        mapping = vec2(1.0, 1.0);
        offset = vec2(radius, radius);
        break;
    }

    gl_Position = transform * vec4(offset, 0.0, 1.0);
}
