#version 300 es
precision highp float;

uniform vec2 u_resolution;
uniform float u_shadow_blur;
uniform float u_shadow_opacity;
uniform vec2 u_shadow_offset;

in vec2 v_tex_coord;
out vec4 frag_color;

void main() {
    vec2 uv = v_tex_coord + u_shadow_offset;
    float shadow = 1.0 - smoothstep(0.0, u_shadow_blur, length(uv - 0.5));
    frag_color = vec4(0.0, 0.0, 0.0, shadow * u_shadow_opacity);
}
