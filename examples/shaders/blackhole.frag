#version 330 core

#define MAX_ITERATIONS 1000
#define STEP_SIZE 0.05

in vec2 TexCoord;
out vec4 fragColor;

uniform vec2 u_resolution;
// texture sampler
uniform sampler2D uSpaceTexture;

vec3 camPos = vec3(0, 0, -10);
vec3 blackholePos = vec3(0, 0, 0);

vec4 raytrace(vec3 rayDir, vec3 rayPos) {
	float h2 = pow(length(cross(rayPos, rayDir)), 2.0);

	for (int i = 0; i < MAX_ITERATIONS; i++) {
	  float dist = length(rayPos - blackholePos);
	  rayDir += -1.5 * h2 * rayPos / pow(pow(dist, 2.0), 2.5) * STEP_SIZE;
	  rayPos += rayDir * STEP_SIZE;
	}

	return texture(uSpaceTexture, rayDir.xy);
}

void main()
{
	vec2 uv = (TexCoord - 0.5) * 2.0 * vec2(u_resolution.x / u_resolution.y, 1);
	vec3 camPos = vec3(0, 0, -10);
	vec3 blackholePos = vec3(0, 0, 0);
	vec3 rayDir = normalize(vec3(uv, 1));
	vec3 rayPos = camPos;
	fragColor = raytrace(rayDir, rayPos);
}

