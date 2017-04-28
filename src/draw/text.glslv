#version 330 core

layout in vec2 position;
layout in vec2 texcoord;

out vec2 tc;

void main() {
  tc = texcoord;
  gl_Position = vec4(position, 0.0, 1.0);
}
