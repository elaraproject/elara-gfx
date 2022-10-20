/*
 * Original shader taken from: https://glslsandbox.com/e#92507.0
 * Not my own work, full credits to the author of the shader,
 * used here just for demonstrative purposes
 */


#ifdef GL_ES
precision highp float;
#endif

uniform float u_time;
uniform vec2 u_resolution;

// --------[ Original ShaderToy begins here ]---------- //
// SST Squelcher

#define AA 1	// make this 2 if you are feeling cold...
#define HEIGHT 12.

vec3 _col;
    
#define PI 3.14159
#define	TAU 6.28318


vec3 spunk(vec2 uv)
{
	vec3 col = vec3(.55,0.35,1.225);		// Drop Colour
	uv.x += sin(20.0+uv.y*1.5)*1.5;
    uv.x = uv.x*50.0;						// H-Count
    float dx = fract(uv.x);
    uv.x = floor(uv.x);
    float t =  u_time*0.04;
    uv.y *= 0.15;							// stretch
    float o=sin(uv.x*215.4);				// offset
    float s=cos(uv.x*33.1)*.3 +.7;			// speed
    float trail = mix(95.0,35.0,s);			// trail length
    float yv = fract(uv.y + t*s + o) * trail;
    yv = 1.0/yv;
    yv = smoothstep(0.0,1.0,yv*yv);
    yv = sin(yv*PI)*(s*5.0);
    float d2 = sin(dx*PI);
    yv *= d2*d2;
    col = col*yv;
	return col;
}

void main(void)
{
     // camera movement	
    float an = sin(u_time*64564.8);
    
    //float dist = 36.0+sin(u_time)*7.0;
    float dist = 200.0;
    
	vec3 ro = vec3( dist*cos(an), sin(u_time*0.75)*14.0, dist*sin(an) );
	//vec3 ro = vec3( 16.0*cos(an), 0.0, 16.0*sin(an) );
    vec3 ta = vec3( 0.0, 0.0, 0.0 );
    // camera matrix
    vec3 ww = normalize( ta - ro );
    vec3 uu = normalize( cross(ww,vec3(0.0,1.0,0.0) ) );
    vec3 vv = normalize( cross(uu,ww));

    vec3 tot = vec3(0.0);
	    vec2 ppp = (-u_resolution.xy + 2.*(gl_FragCoord.xy))/u_resolution.y;

	vec3 bbk = spunk(ppp.xy);
	
        vec2 p = (-u_resolution.xy + 2.0*gl_FragCoord.xy)/u_resolution.y;

	// create view ray
        vec3 rd = normalize( p.x*uu + p.y*vv + 1.5*ww );

        // raymarch
        const float tmax = 65.0;
        float t = 0.0;
        for( int i=0; i<100; i++ )
        {
            vec3 pos = ro + t*rd;

        }
    
        // shading/lighting	
        float v = 1.0-abs(p.y);
        vec3 col = bbk*v*2.0;	//vec3(v*0.1);

        if( t<tmax )
        {
            vec3 pos = ro + t*rd;

            
            vec3 dir = normalize(vec3(1.0,0.7,0.0));
        }
        // gamma        
        col = sqrt( col );
	tot += col;
    
	    
	gl_FragColor = vec4( tot, 1.0 );

}