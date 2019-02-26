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
  float texLeft;
  float texTop;
  float texWidth;
  float texHeight;
  vec4 tint;
} pushed;

layout(location = 0) out vec4 outColor;
layout(location = 1) out vec2 outTexCoord;

void main() {
  outColor = pushed.tint;

  switch (gl_VertexIndex) {
    case 0:
      gl_Position = pushed.transform * vec4(pushed.left, pushed.top, 0.0, 1.0);
      outTexCoord = vec2(pushed.texLeft, pushed.texTop);
      break;
    case 1:
      gl_Position = pushed.transform * vec4(pushed.left, pushed.top + pushed.height, 0.0, 1.0);
      outTexCoord = vec2(pushed.texLeft, pushed.texTop + pushed.texHeight);
      break;
    case 2:
      gl_Position = pushed.transform * vec4(pushed.left + pushed.width, pushed.top, 0.0, 1.0);
      outTexCoord = vec2(pushed.texLeft + pushed.texWidth, pushed.texTop);
      break;
    case 3:
      gl_Position = pushed.transform * vec4(pushed.left + pushed.width, pushed.top + pushed.height, 0.0, 1.0);
      outTexCoord = vec2(pushed.texLeft + pushed.texWidth, pushed.texTop + pushed.texHeight);
      break;
  }
}
