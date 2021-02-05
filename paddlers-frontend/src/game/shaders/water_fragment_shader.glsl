varying highp vec2 Tex_coord;
uniform sampler2D sampler;
void main() {
    highp vec4 tex_color = texture2D(sampler, Tex_coord);
    gl_FragColor = tex_color;
}