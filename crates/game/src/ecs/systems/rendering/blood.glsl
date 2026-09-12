#version 100

precision highp float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    vec4 color = texture2D(Texture, uv);

    color.r *= 3.0;

    gl_FragColor = color;
}
