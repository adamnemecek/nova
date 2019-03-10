#version 450

layout(location = 0) in vec4 inColor;
layout(location = 1) in vec2 inTexCoord;

layout(location = 0) out vec4 outColor;

layout(binding = 0) uniform sampler2D texSampler;

void main() {
  float r = texture(texSampler, inTexCoord).r;

  outColor = inColor * vec4(r, r, r, r);
}
