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

    if (sampleUv.x < 0.0 || sampleUv.x > 1.0 ||
        sampleUv.y < 0.0 || sampleUv.y > 1.0) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }

    vec4 color = texture2D(Texture, sampleUv);

    vec2 texel = 1.0 / screen_size;
    float threshold = 0.65;
    float bloom_size = 4.0;

    vec3 bloom = vec3(0.0);

    // helper-style inline: sample, threshold, accumulate
    vec2 offsets[8];
    offsets[0] = vec2(-bloom_size, -bloom_size);
    offsets[1] = vec2( 0.0,        -bloom_size);
    offsets[2] = vec2( bloom_size, -bloom_size);
    offsets[3] = vec2(-bloom_size,  0.0);
    offsets[4] = vec2( bloom_size,  0.0);
    offsets[5] = vec2(-bloom_size,  bloom_size);
    offsets[6] = vec2( 0.0,         bloom_size);
    offsets[7] = vec2( bloom_size,  bloom_size);

    for (int i = 0; i < 8; i++) {
        vec3 s = texture2D(Texture, sampleUv + texel * offsets[i]).rgb;
        float b = max(s.r, max(s.g, s.b));
        float mask = clamp((b - threshold) / max(1.0 - threshold, 0.001), 0.0, 1.0);
        bloom += s * mask;
    }

    bloom /= 8.0;

    // Bloom intensity
    color.rgb += bloom * 0.35;

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
