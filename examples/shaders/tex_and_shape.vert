#version 100
attribute highp vec2 position;
attribute highp vec2 tex_coord;
attribute lowp vec4 vertex_color;
varying highp vec2 TexCoord;
varying lowp vec4 VertexColor;
uniform float aspect_ratio;

void main() {
    VertexColor = vertex_color;
    TexCoord = tex_coord;
    gl_Position = vec4(position.x * aspect_ratio, position.y, 0.0, 1.0);
}