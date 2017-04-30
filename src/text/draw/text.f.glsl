#version 330 core

in vec2 TexCoords;

out vec4 out_color;

uniform sampler2D font;

uniform Locals {
  mat4 transform;
  vec3 color;
};

void main() {
  float font_brightness = texture(font, TexCoords).r;
  if (font_brightness < 0.001) {
    discard;
  }
  out_color = font_brightness * vec4(color.xyz, 1.0);
}
