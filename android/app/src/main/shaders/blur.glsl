#version 300 es
precision highp float;

uniform sampler2D u_texture;
uniform vec2 u_resolution;
uniform float u_blur_radius;

in vec2 v_tex_coord;
out vec4 frag_color;

void main() {
    vec2 texel_size = 1.0 / u_resolution;
    vec4 color = vec4(0.0);
    float total = 0.0;
    
    for (float x = -u_blur_radius; x <= u_blur_radius; x += 1.0) {
        for (float y = -u_blur_radius; y <= u_blur_radius; y += 1.0) {
            vec2 offset = vec2(x, y) * texel_size;
            float weight = 1.0 / (1.0 + length(vec2(x, y)));
            color += texture(u_texture, v_tex_coord + offset) * weight;
            total += weight;
        }
    }
    
    frag_color = color / total;
}
