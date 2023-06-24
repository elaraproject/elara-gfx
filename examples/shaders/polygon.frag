#version 330 core
in vec2 TexCoord;
in vec4 VertexColor;
uniform sampler2D uTexture;
out vec4 FragColor;

void main() {
    vec4 col = texture(uTexture, TexCoord);
    FragColor = mix(VertexColor.rgba, col, col.a);
}