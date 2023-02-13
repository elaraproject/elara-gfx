// Taken from https://stackoverflow.com/questions/63408121/opengl-simple-antialiased-polygon-grid-shader
#version 330

#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution.xy * vec2(u_resolution.x / u_resolution.y, 1.0);
    // get some diagonal lines going
    uv.yx += uv.xy * 0.1;

    // for every unit of texture space, I want 10 grid lines
    float gridSize = 10.0;
    // width of a line on the screen plus a little bit for AA
    float width = (gridSize * 1.2) / u_resolution.y;

    // chop up into grid
    uv = fract(uv * gridSize);
    // abs version
    float grid = max(
        1.0 - abs((uv.y - 0.5) / width),
        1.0 - abs((uv.x - 0.5) / width)
    );

    // Output to screen (for shadertoy only)
    gl_FragColor = vec4(grid, grid, grid, 1.0);
}

