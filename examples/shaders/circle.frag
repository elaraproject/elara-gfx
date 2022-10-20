#version 330

uniform float u_time;
uniform vec2 u_resolution;
out vec4 frag_color;

#define PI 3.14159
#define	TAU 6.28318

void main()
{
	vec2 p = (2.0*gl_FragCoord.xy-u_resolution.xy)/u_resolution.y;
    float a = atan(p.x,p.y);
    float r = length(p)*0.75;
    vec2 uv = vec2(a/TAU,r);
	
	//get the color
	float xCol = (uv.x - (u_time / 3.0)) * 3.0;
	xCol = mod(xCol, 3.0);
	vec3 horColour = vec3(0.25, 0.25, 0.25);
	
	if (xCol < 1.0) {
		
		horColour.r += 1.0 - xCol;
		horColour.g += xCol;
	}
	else if (xCol < 2.0) {
		
		xCol -= 1.0;
		horColour.g += 1.0 - xCol;
		horColour.b += xCol;
	}
	else {
		
		xCol -= 2.0;
		horColour.b += 1.0 - xCol;
		horColour.r += xCol;
	}

	// draw color beam
	uv = (2.0 * uv) - 1.0;
	float beamWidth = (0.7+0.5*cos(uv.x*10.0*TAU*0.15*clamp(floor(5.0 + 10.0*cos(u_time)), 0.0, 10.0))) * abs(1.0 / (30.0 * uv.y));
	vec3 horBeam = vec3(beamWidth);
	frag_color = vec4((( horBeam) * horColour), 1.0);
}