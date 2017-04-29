#version 330 core

in vec2 tc;

out vec4 out_color;

uniform sampler2D font;

uniform Locals {
  mat4 transform;
  vec4 color;
};

void main() {
  float brightness = texture(font, tc).r;
  out_color = brightness * color;
}
