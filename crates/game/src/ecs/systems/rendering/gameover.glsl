#version 100

precision highp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 screen_size;
uniform vec2 center;
uniform float radius;

void main() {
    vec2 pos = uv * screen_size;
    vec4 color = texture2D(Texture, uv);

    vec2 delta = pos - center;
    float d2 = dot(delta, delta);

    if (d2 > radius * radius) {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    }

    gl_FragColor = color;
}
