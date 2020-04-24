use super::utils::*;
use quicksilver::geom::Rectangle;

#[test]
fn shrink_rectangle_to_center() {
    let frame = Rectangle::new((-80, -80), (160, 160));
    let padded = frame.shrink_to_center(0.875);

    assert_eq!(padded, Rectangle::new((-70, -70), (140, 140)));
}

#[test]
fn padded_rectangle() {
    let frame = Rectangle::new((-50, -50), (100, 100));
    let padded = frame.padded(5.0);

    assert_eq!(padded, Rectangle::new((-45, -45), (90, 90)));
}

#[test]
fn fit_rectangle_into_rectangle() {
    let frame = Rectangle::new((-50, -50), (100, 100));
    let rect = Rectangle::new((111, 222), (200, 400));
    let top_left = rect.fit_into(&frame, FitStrategy::TopLeft);
    let centered = rect.fit_into(&frame, FitStrategy::Center);

    assert_eq!(top_left, Rectangle::new((-50, -50), (50, 100)));
    assert_eq!(centered, Rectangle::new((-25, -50), (50, 100)));
}

#[test]
fn shrink_fit_rectangle_into_rectangle() {
    let frame = Rectangle::new((-400, -400), (800, 800));
    let rect = Rectangle::new((111, 222), (200, 400));
    let top_left = rect.shrink_and_fit_into(&frame, FitStrategy::TopLeft);
    let centered = rect.shrink_and_fit_into(&frame, FitStrategy::Center);

    assert_eq!(top_left, Rectangle::new((-400, -400), (200, 400)));
    assert_eq!(centered, Rectangle::new((-100, -200), (200, 400)));
}

#[test]
fn grow_fit_rectangle_into_rectangle() {
    let frame = Rectangle::new((-400, -400), (800, 800));
    let rect = Rectangle::new((111, 222), (200, 400));
    let top_left = rect.fit_into(&frame, FitStrategy::TopLeft);
    let centered = rect.fit_into(&frame, FitStrategy::Center);

    assert_eq!(top_left, Rectangle::new((-400, -400), (400, 800)));
    assert_eq!(centered, Rectangle::new((-200, -400), (400, 800)));
}

#[test]
fn grid_within_rectangle() {
    let frame = Rectangle::new((0, 0), (600, 500));
    let grid: Vec<Rectangle> = frame.grid(3, 2).collect();

    let solution = vec![
        Rectangle::new((0, 0), (200, 250)),
        Rectangle::new((200, 0), (200, 250)),
        Rectangle::new((400, 0), (200, 250)),
        Rectangle::new((0, 250), (200, 250)),
        Rectangle::new((200, 250), (200, 250)),
        Rectangle::new((400, 250), (200, 250)),
    ];

    assert_eq!(grid.len(), solution.len());

    for (l, r) in grid.iter().zip(solution.iter()) {
        assert_eq!(l, r);
    }
}

#[test]
fn fit_square_in_rectangle() {
    let frame = Rectangle::new((1000, 1000), (100, 1000));

    let top_left = frame.fit_square(FitStrategy::TopLeft);
    let center = frame.fit_square(FitStrategy::Center);

    assert_eq!(
        top_left,
        Rectangle::new((1000, 1000), (100, 100)),
        "Fitting rectangle at top left"
    );
    assert_eq!(
        center,
        Rectangle::new((1000, 1450), (100, 100)),
        "Fitting rectangle in center"
    );
}

#[test]
fn cut_horizontal() {
    let a = Rectangle::new((100, 100), (100, 100));
    let h = 30.0;

    let (b, c) = a.cut_horizontal(h);

    assert_eq!(b, Rectangle::new((100, 100), (100, 30)));
    assert_eq!(c, Rectangle::new((100, 130), (100, 70)));
}

#[test]
fn cut_vertical() {
    let a = Rectangle::new((100, 100), (100, 100));
    let w = 75.0;

    let (b, c) = a.cut_vertical(w);

    assert_eq!(b, Rectangle::new((100, 100), (75, 100)));
    assert_eq!(c, Rectangle::new((175, 100), (25, 100)));
}
