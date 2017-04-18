#version 330 core

uniform Locals {
  mat4 transform;
  vec4 color;
};

out vec4 out_color;

void main() { out_color = color; }
