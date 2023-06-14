#version 150 core

in vec3 Color;

void main()
{
    gl_FragColor = vec4(Color, 1.0);
}
