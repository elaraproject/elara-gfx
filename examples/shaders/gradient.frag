#version 330

uniform float u_time;
uniform vec2 u_resolution;
out vec4 frag_color;

void main()
{
    // Normalized pixel coordinates (from 0 to 1)
    // vec2 uv = gl_FragCoord.xy/u_resolution;
    // Output to screen
    // gl_FragColor = vec4(gl_FragCoord.xy / 500.0, 1.0, 1.0);
    gl_FragColor = vec4(gl_FragCoord.xy / u_resolution, 1.0, 1.0);
}