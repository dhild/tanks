#version 330 core

in vec2 position;
in vec2 texcoord;

out vec2 tc;

uniform Locals {
  mat4 transform;
  vec4 color;
};

void main() {
  tc = texcoord;
  gl_Position = transform * vec4(position, 0.0, 1.0);
}
