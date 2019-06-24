#version 450

layout(binding = 1) uniform sampler2D texSampler;

layout(location = 0) in vec4 inColor;
layout(location = 1) in vec2 inTexCoord;

layout(location = 0) out vec4 outColor;

layout(push_constant) uniform PushConstants {
  vec4 tint;
} pushed;

void main() {
  outColor = texture(texSampler, inTexCoord) * inColor;
}
