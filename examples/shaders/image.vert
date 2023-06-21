#version 330 core
in vec2 position;
in vec2 tex_coord;

out vec2 TexCoord;

void main()
{
	gl_Position = vec4(position.x * 0.66, position.y, 0.0, 1.0);
	TexCoord = vec2(tex_coord.x, tex_coord.y);
}

