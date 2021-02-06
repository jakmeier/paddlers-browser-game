varying highp vec2 Tex_coord; // This is stretched across the full river area, but the actual texture is tiled.
varying highp vec2 Coordinate; // This is the position in the full screen
uniform sampler2D sampler;
uniform mediump float Time;
void main() {
    highp float v = 0.2;
    highp float big_tile_size = 0.05;
    highp float small_tile_size = 0.01;
    highp vec2 tex_coord = mod(Tex_coord / big_tile_size, 1.0);
    highp vec2 offset = vec2(0.1 * cos(Time+10.0*Coordinate.x)) + vec2(v * Time, 0.0);
    offset.x += 2.0 * Tex_coord.y;
    offset.y += sin(length(Coordinate / small_tile_size)) * small_tile_size;
    highp vec2 p = mod(tex_coord + offset, 1.0);
    highp vec4 tex_color = texture2D(sampler, vec2(p.y, p.x));
    gl_FragColor = tex_color;
}