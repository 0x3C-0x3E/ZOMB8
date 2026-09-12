#version 100

precision highp float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    vec4 color = texture2D(Texture, uv);

    color.r *= 1.2;
    color.b *= 0.3;
    color.g *= 0.3;

    gl_FragColor = color;
}
