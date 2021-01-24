// Credits for perlin noise go to https://www.shadertoy.com/view/tdXBW4#
precision mediump float;
vec2 hash22(vec2 p)
{
	p = vec2(dot(p, vec2(127.1, 311.7)),
		dot(p, vec2(269.5, 183.3)));
	return -1.0 + 2.0 * fract(sin(p)*43758.5453123);
}

float perlin_noise(vec2 p)
{
	vec2 pi = floor(p);
	vec2 pf = p - pi;
	vec2 w = pf * pf * (3.0 - 2.0 * pf);
	return mix(mix(dot(hash22(pi + vec2(0.0, 0.0)), pf - vec2(0.0, 0.0)),
		dot(hash22(pi + vec2(1.0, 0.0)), pf - vec2(1.0, 0.0)), w.x),
		mix(dot(hash22(pi + vec2(0.0, 1.0)), pf - vec2(0.0, 1.0)),
			dot(hash22(pi + vec2(1.0, 1.0)), pf - vec2(1.0, 1.0)), w.x),
		w.y);
}
// Fractal Brownian Motion implementation based on [The book of shaders](https://thebookofshaders.com/)
#define NUM_OCTAVES 5
float fbm ( in vec2 _st) {
    float v = 0.000;
    float a = 0.500;
    vec2 shift = vec2(100.0);
    // Rotate to reduce axial bias
    mat2 rot = mat2(cos(0.5), sin(0.5),
                    -sin(0.5), cos(0.50));
    for (int i = 0; i < NUM_OCTAVES; ++i) {
        v += a * perlin_noise(_st);
        _st = rot * _st * 2.0 + shift;
        a *= 0.5;
    }
    return v;
}

varying highp vec2 Coordinate;
uniform float Time;
void main() {
	float seed = 10.7;
    float vivid = 0.5;
    float bounce = 0.5;
    
    vec2 pattern_noise_input = 20.0 * Coordinate + vec2(seed);
    vec2 local_offset = vec2(bounce * cos(Time+3.0*length(Coordinate)));
    pattern_noise_input += local_offset;
    vec3 color = abs(vec3(1.0,1.0,1.0));
    float t = fbm(vec2(Time, Time));
    color = color * perlin_noise(pattern_noise_input);
    
    vec2 r = vec2(0.); 
    r.x = fbm( Coordinate * 5.0 + vivid * Time );
    r.y = fbm( Coordinate * 5.0 + vivid * Time);


    float f = fbm(Coordinate * 3.0 + r);
    color = ( 3.0*f*f + 3.5*f ) * color;
    
    gl_FragColor = vec4(color, 1.0);
}