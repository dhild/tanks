#version 330 core

layout(location = 0) in vec2 position;

uniform Locals {
  mat4 transform;
  vec4 color;
};

void main() {
  gl_Position = transform * vec4(position, 0.0, 1.0);
}
