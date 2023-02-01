#version 330

uniform vec2 u_resolution;

void main()
{
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = gl_FragCoord.xy / u_resolution;
    // Output to screen
    gl_FragColor = vec4(uv, 1.0, 1.0);
}
