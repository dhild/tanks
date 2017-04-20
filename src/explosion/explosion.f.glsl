#version 330 core

layout(std140) uniform;

in vec2 mapping;

uniform Locals {
  mat4 transform;
  float radius;
};

out vec4 out_color;

void main() {
  float lensqr = dot(mapping, mapping);
  if (lensqr > 1.0)
    discard;

  out_color = vec4(1.0, 1.0 - lensqr, 0.0, 0.1);
}
