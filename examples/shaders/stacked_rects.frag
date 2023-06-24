#version 100
varying highp vec2 TexCoord;
varying lowp vec4 VertexColor;
uniform sampler2D uTexture;

void main() {
    lowp vec4 col = texture2D(uTexture, TexCoord);
    gl_FragColor = mix(VertexColor.rgba, col, col.a);
}