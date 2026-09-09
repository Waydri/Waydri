#version 320 es
precision mediump float;

in vec2 vTexCoord;

uniform sampler2D uTexture;
uniform vec4 uRect;
uniform float uRadius;
uniform vec2 uResolution;
uniform float uShadowSize;
uniform float uShadowAlpha;

out vec4 fragColor;

float roundedBoxSDF(vec2 center, vec2 halfSize, float radius) {
    return length(max(abs(center) - halfSize + radius, 0.0)) - radius;
}

float gaussianWeight(float x, float sigma) {
    float sq = sigma * sigma;
    return exp(-(x * x) / (2.0 * sq)) / (sqrt(6.2831853) * sigma);
}

void main() {
    vec2 pixelPos = vTexCoord * uResolution;
    vec2 rectCenter = uRect.xy + uRect.zw * 0.5;
    vec2 halfSize = uRect.zw * 0.5;
    float dist = roundedBoxSDF(pixelPos - rectCenter, halfSize, uRadius);
    float alphaInner = 1.0 - smoothstep(-1.0, 0.0, dist);
    vec4 texColor = texture(uTexture, vTexCoord);

    float shadowDist = -dist;
    float shadowRadius = uShadowSize;
    float shadowMask = smoothstep(0.0, shadowRadius, shadowDist);
    shadowMask *= gaussianWeight(shadowDist, shadowRadius * 0.5);
    shadowMask = clamp(shadowMask * uShadowAlpha, 0.0, 1.0);
    vec4 shadowColor = vec4(0.0, 0.0, 0.0, shadowMask);

    vec4 finalColor = mix(shadowColor, texColor, alphaInner);
    finalColor.a = max(finalColor.a, shadowColor.a * (1.0 - texColor.a));

    fragColor = finalColor;
}
