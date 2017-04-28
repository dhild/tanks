#version 330 core

in vec2 tc;

out vec4 out_color;

uniform sampler2D fontTexture;

uniform Color {
  vec4 color;
};

void main() {
  float brightness = texture(fontTexture, tc).r;
  out_color = brightness * color;
}
