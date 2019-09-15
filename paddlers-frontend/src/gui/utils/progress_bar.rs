use super::*;
use quicksilver::prelude::*;

pub fn draw_progress_bar(
    window: &mut Window,
    asset: &mut Asset<Font>,
    area: Rectangle,
    progress: f32,
    text: &str,
) {
    let text_h = (area.height() * 0.5).max(50.0);
    let (text_area, bar_area) = area.cut_horizontal(text_h);

    let z = 1;

    write_text_col(
        asset,
        window,
        &text_area.padded(10.0),
        z,
        FitStrategy::Center,
        &text,
        Color::WHITE,
    )
    .unwrap();

    window.draw(&bar_area, Col(Color::WHITE));
    let mut bar = bar_area.padded(3.0);
    window.draw(&bar, Col(DARK_GREEN));
    bar.size.x *= progress;
    window.draw(&bar, Col(LIME_GREEN));
}
