#version 300 es
precision highp float;

uniform vec2 u_resolution;
uniform vec4 u_color1;
uniform vec4 u_color2;
uniform float u_angle;

in vec2 v_tex_coord;
out vec4 frag_color;

void main() {
    vec2 pos = v_tex_coord;
    float gradient = pos.x * cos(u_angle) + pos.y * sin(u_angle);
    frag_color = mix(u_color1, u_color2, gradient);
}
