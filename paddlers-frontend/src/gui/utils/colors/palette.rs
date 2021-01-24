use crate::gui::utils::*;
use paddle::quicksilver_compat::*;
pub fn draw_color_palette(window: &mut DisplayArea, area: Rectangle) {
    let (green, blue) = area.cut_horizontal(area.height() / 2.0);
    draw_three_colors(window, green, LIGHT_GREEN, GREEN, DARK_GREEN);
    draw_three_colors(window, blue, LIGHT_BLUE, BLUE, DARK_BLUE);
}

fn draw_three_colors(
    window: &mut DisplayArea,
    mut area: Rectangle,
    col1: Color,
    col2: Color,
    col3: Color,
) {
    let w = area.size.x / 3.0;
    area.size.x = w;
    window.draw_ex(&area, &col1, Transform::IDENTITY, 1000);
    area.pos.x += w;
    window.draw_ex(&area, &col2, Transform::IDENTITY, 1000);
    area.pos.x += w;
    window.draw_ex(&area, &col3, Transform::IDENTITY, 1000);
}
