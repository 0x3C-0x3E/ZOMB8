#version 100

precision highp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 screen_size;
uniform float time;
uniform float strength;

void main() {
    vec2 centered = uv * 2.0 - 1.0;
    float aspect = screen_size.x / screen_size.y;

    // --- Fisheye distortion (aspect-corrected, unchanged) ---
    vec2 fisheyeCentered = centered;
    fisheyeCentered.x *= aspect;

    float dFisheye = length(fisheyeCentered);
    float distortion = 1.0 + strength * dFisheye * dFisheye;
    vec2 distorted = fisheyeCentered * distortion;

    distorted.x /= aspect;
    vec2 sampleUv = (distorted + 1.0) * 0.5;

    if (sampleUv.x < 0.0 || sampleUv.x > 1.0 || sampleUv.y < 0.0 || sampleUv.y > 1.0) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }

    vec4 color = texture2D(Texture, sampleUv);

    // --- Scanlines ---
    float scanline = sin(sampleUv.y * 800.0 * 3.14159) * 0.5 + 0.5;
    color.rgb *= mix(0.85, 1.0, scanline);

    // --- Vignette (NOT aspect-corrected, stays consistent across screen shapes) ---
    float dVignette = length(centered);
    float vignette = 1.0 - dVignette * dVignette * 0.35;
    color.rgb *= vignette;

    // --- Flicker ---
    // float flicker = 0.98 + 0.02 * sin(time * 20.0);
    // color.rgb *= flicker;

    gl_FragColor = color;
}
