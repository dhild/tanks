#version 330 core

layout(location = 0) in vec2 position;

uniform Bounds {
  mat4 transform;
};

void main() {
  gl_Position = transform * vec4(position.x, position.y, 0.0, 1.0);
}
