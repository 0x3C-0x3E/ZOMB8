#version 100

precision highp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 screen_size;
uniform float time;

void main() {
    vec4 color = texture2D(Texture, uv);

    // Scanlines
    float scanline = sin(uv.y * screen_size.y * 3.14159) * 0.5 + 0.5;
    color.rgb *= mix(0.85, 1.0, scanline);

    // Vignette (darken edges)
    vec2 centered = uv * 2.0 - 1.0;
    float vignette = 1.0 - dot(centered, centered) * 0.35;
    color.rgb *= vignette;

    // Subtle flicker
    float flicker = 0.98 + 0.02 * sin(time * 15.0);
    color.rgb *= flicker;

    gl_FragColor = color;
}
