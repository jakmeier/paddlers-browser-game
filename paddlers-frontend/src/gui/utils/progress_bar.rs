use super::*;
use paddle::quicksilver_compat::*;
use paddle::NutsCheck;
use paddle::*;

pub fn draw_progress_bar(
    window: &mut DisplayArea,
    float: &mut FloatingText,
    area: Rectangle,
    progress: f32,
    text: &str,
) -> PadlResult<()> {
    let text_h = (area.height() * 0.5).max(50.0);
    let (text_area, bar_area) = area.cut_horizontal(text_h);

    let z = 1;

    float
        .write(
            window,
            &text_area.padded(10.0),
            z,
            FitStrategy::Center,
            text,
        )
        .nuts_check();

    window.draw(&bar_area, Color::WHITE);
    let mut bar = bar_area.padded(3.0);
    window.draw(&bar, DARK_GREEN);
    bar.size.x *= progress;
    window.draw(&bar, LIGHT_GREEN);
    Ok(())
}
