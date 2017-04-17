#version 330 core

layout(lines) in;
layout(triangle_strip, max_vertices = 4) out;

uniform Base {
  float base_y;
};

vec4 base_vec(vec4 top) {
  return vec4(top.x, base_y, top.z, top.w);
}

void main() {
  gl_Position = gl_in[0].gl_Position;
  EmitVertex();
  gl_Position = base_vec(gl_in[0].gl_Position);
  EmitVertex();
  gl_Position = gl_in[1].gl_Position;
  EmitVertex();
  gl_Position = base_vec(gl_in[1].gl_Position);
  EmitVertex();
  EndPrimitive();
}
