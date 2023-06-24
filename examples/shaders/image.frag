#version 330 core

in vec2 TexCoord;
out vec4 fragColor;

// texture sampler
uniform sampler2D texture1;

void main()
{
	fragColor = texture(texture1, TexCoord);
}

