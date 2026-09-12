#version 100

precision highp float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    vec4 color = texture2D(Texture, uv);

    color.r *= 1.8;
    color.b *= 0.8;
    color.g *= 0.8;

    gl_FragColor = color;
}
