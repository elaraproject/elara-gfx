#version 330 core
in vec2 position;
in vec2 tex_coord;
in vec4 vertex_color;
out vec2 TexCoord;
out vec4 VertexColor;
uniform float aspect_ratio;

void main() {
    VertexColor = vertex_color;
    TexCoord = tex_coord;
    gl_Position = vec4(position.x * aspect_ratio, position.y, 0.0, 1.0);
}