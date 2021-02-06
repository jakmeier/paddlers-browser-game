use crate::gui::sprites::{SingleSprite, SpriteIndex, Sprites};
use paddle::{CustomShader, Display, Image, UniformValue, VertexDescriptor};

pub const VERTEX_SHADER: &str = include_str!("shaders/vertex_shader.glsl");
pub const VERTEX_SHADER_WITH_TEX: &str = include_str!("shaders/vertex_shader_with_tex.glsl");

pub const RELIGION_FRAGMENT_SHADER: &str = include_str!("shaders/religion_fragment_shader.glsl");
pub const WATER_FRAGMENT_SHADER: &str = include_str!("shaders/water_fragment_shader.glsl");

pub struct Shaders {
    pub religion_background: CustomShader,
    pub water: CustomShader,
}

impl Shaders {
    pub fn load(display: &mut Display, sprites: &Sprites) -> Self {
        Self {
            religion_background: religion_background_shader(display),
            water: water_shader(
                display,
                sprites.index(SpriteIndex::Simple(SingleSprite::Water)),
            ),
        }
    }
}
fn religion_background_shader(display: &mut Display) -> CustomShader {
    let vertex_shader = VERTEX_SHADER;
    let fragment_shader = RELIGION_FRAGMENT_SHADER;
    let vertex_descriptor = VertexDescriptor::new().with_pos();

    let projection = display.webgl_transform();

    let uniform_values = &[
        ("Projection", UniformValue::Matrix3fv(projection.as_slice())),
        ("Time", UniformValue::F32(0.0)),
    ];

    CustomShader::new(
        display
            .new_render_pipeline(
                vertex_shader,
                fragment_shader,
                vertex_descriptor,
                uniform_values,
            )
            .expect("Loading shader failed"),
    )
}
fn water_shader(display: &mut Display, water_texture: Image) -> CustomShader {
    let vertex_shader = VERTEX_SHADER_WITH_TEX;
    let fragment_shader = WATER_FRAGMENT_SHADER;
    let vertex_descriptor = VertexDescriptor::new().with_pos().with_tex();

    let projection = display.webgl_transform();

    let uniform_values = &[
        ("Projection", UniformValue::Matrix3fv(projection.as_slice())),
        ("Time", UniformValue::F32(0.0)),
    ];

    CustomShader::new(
        display
            .new_render_pipeline(
                vertex_shader,
                fragment_shader,
                vertex_descriptor,
                uniform_values,
            )
            .expect("Loading shader failed"),
    )
    .with_image(water_texture)
}
