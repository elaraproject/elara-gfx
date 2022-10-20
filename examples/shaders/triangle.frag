#version 330 core
out vec4 Color;
varying vec3 v_color;

void main() {
    Color = vec4(v_color, 1.0);
}
