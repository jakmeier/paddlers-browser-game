use paddle::{CustomShader, Display, UniformValue, VertexDescriptor};

pub const VERTEX_SHADER: &str = include_str!("shaders/vertex_shader.glsl");
pub const RELIGION_FRAGMENT_SHADER: &str = include_str!("shaders/religion_fragment_shader.glsl");

pub struct Shaders {
    pub religion_background: CustomShader,
}

impl Shaders {
    pub fn load(display: &mut Display) -> Self {
        Self {
            religion_background: religion_background_shader(display),
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
