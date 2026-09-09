#version 320 es
precision mediump float;
precision mediump int;

#ifdef VERTEX_SHADER
layout(location = 0) in vec4 aPosition;
layout(location = 1) in vec2 aTexCoord;

uniform mat4 uProjection;
uniform mat4 uModelView;

out vec2 vTexCoord;

void main() {
    vTexCoord = aTexCoord;
    gl_Position = uProjection * uModelView * aPosition;
}
#endif

#ifdef FRAGMENT_SHADER
in vec2 vTexCoord;

uniform sampler2D uTexture;
uniform sampler2D uTextureNext;
uniform float uProgress;
uniform int uEffectType;
uniform float uAlpha;

out vec4 fragColor;

vec2 ease(vec2 t) {
    return t * t * (3.0 - 2.0 * t);
}

vec2 reverseEase(vec2 t) {
    vec2 o = vec2(1.0) - t;
    return vec2(1.0) - o * o;
}

void main() {
    vec2 uv = vTexCoord;
    vec4 color = vec4(0.0);
    float t = clamp(uProgress, 0.0, 1.0);
    float eased = 1.0 - (1.0 - t) * (1.0 - t);

    if (uEffectType == 0) {
        vec4 fromColor = texture(uTexture, uv);
        vec4 toColor = texture(uTextureNext, uv);
        color = mix(fromColor, toColor, eased);
    } else if (uEffectType == 1) {
        vec2 offset = vec2(eased, 0.0);
        vec2 uvFrom = uv + offset;
        vec2 uvTo = uv - vec2(1.0 - eased, 0.0);
        uvFrom = clamp(uvFrom, vec2(0.0), vec2(1.0));
        uvTo = clamp(uvTo, vec2(0.0), vec2(1.0));
        vec4 fromColor = texture(uTexture, uvFrom);
        vec4 toColor = texture(uTextureNext, uvTo);
        float edge = smoothstep(0.45, 0.55, uv.x);
        color = mix(toColor, fromColor, edge);
    } else if (uEffectType == 2) {
        vec2 offset = vec2(-eased, 0.0);
        vec2 uvFrom = uv + vec2(1.0 - eased, 0.0);
        vec2 uvTo = uv - offset;
        uvFrom = clamp(uvFrom, vec2(0.0), vec2(1.0));
        uvTo = clamp(uvTo, vec2(0.0), vec2(1.0));
        vec4 fromColor = texture(uTexture, uvFrom);
        vec4 toColor = texture(uTextureNext, uvTo);
        float edge = smoothstep(0.45, 0.55, uv.x);
        color = mix(toColor, fromColor, 1.0 - edge);
    } else if (uEffectType == 3) {
        float scale = mix(0.5, 1.0, eased);
        vec2 centered = (uv - vec2(0.5)) / scale + vec2(0.5);
        centered = clamp(centered, vec2(0.0), vec2(1.0));
        vec4 fromColor = texture(uTexture, uv);
        vec4 toColor = texture(uTextureNext, centered);
        float mask = step(0.0, centered.x) * step(centered.x, 1.0) *
                     step(0.0, centered.y) * step(centered.y, 1.0);
        color = mix(fromColor, toColor * mask, eased);
    }

    fragColor = color * uAlpha;
}
#endif
