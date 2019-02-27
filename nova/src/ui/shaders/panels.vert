#version 450

vec2 texCoords[4] = vec2[] (
  vec2(0.0, 0.0),
  vec2(0.0, 1.0),
  vec2(1.0, 0.0),
  vec2(1.0, 1.0)
);

layout(push_constant) uniform PushConstants {
  mat4 transform;
  float left;
  float top;
  float width;
  float height;
  vec2 tex1;
  vec2 tex2;
  vec4 tint;
} pushed;

layout(location = 0) out vec4 outColor;
layout(location = 1) out vec2 outTexCoord;

void main() {
  outColor = pushed.tint;

  switch (gl_VertexIndex) {
    case 0:
      gl_Position = pushed.transform * vec4(pushed.left, pushed.top, 0.0, 1.0);
      outTexCoord = pushed.tex1;
      break;
    case 1:
      gl_Position = pushed.transform * vec4(pushed.left, pushed.top + pushed.height, 0.0, 1.0);
      outTexCoord = vec2(pushed.tex1.x, pushed.tex2.y);
      break;
    case 2:
      gl_Position = pushed.transform * vec4(pushed.left + pushed.width, pushed.top, 0.0, 1.0);
      outTexCoord = vec2(pushed.tex2.x, pushed.tex1.y);
      break;
    case 3:
      gl_Position = pushed.transform * vec4(pushed.left + pushed.width, pushed.top + pushed.height, 0.0, 1.0);
      outTexCoord = pushed.tex2;
      break;
  }
}
