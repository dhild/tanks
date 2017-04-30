#version 330 core

in vec2 position;
in vec2 texcoord;

out vec2 TexCoords;

uniform Locals {
  mat4 transform;
  vec3 color;
};

void main() {
  TexCoords = texcoord;
  gl_Position = transform * vec4(position, 0.0, 1.0);
}
