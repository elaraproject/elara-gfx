#version 330 core
in vec2 position;
in vec3 color;

uniform float aspect_ratio;

out vec3 Color;

void main() {
    Color = color;
    gl_Position = vec4(position.x * aspect_ratio, position.y, 0.0, 1.0);
}
