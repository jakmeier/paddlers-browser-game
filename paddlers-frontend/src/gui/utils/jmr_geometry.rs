use quicksilver::prelude::*;
use super::*;

pub trait JmrRectangle
where
    Self: std::marker::Copy,
{
    #[must_use]
    fn shrink_to_center(&self, shrink_to_center: f32) -> Rectangle;
    #[must_use]
    fn padded(&self, padding: f32) -> Rectangle;
    #[must_use]
    fn fit_into_ex(self, frame: &Rectangle, fit_strat: FitStrategy, allow_grow: bool) -> Rectangle;
    #[must_use]
    fn fit_square(&self, fit_strat: FitStrategy) -> Rectangle;
    #[must_use]
    fn grid(&self, cols: usize, rows: usize) -> Grid;
    #[must_use]
    fn cut_horizontal(&self, h: f32) -> (Rectangle, Rectangle);
    #[must_use]
    fn cut_vertical(&self, h: f32) -> (Rectangle, Rectangle);
    #[must_use]
    fn shrink_and_fit_into(self, frame: &Rectangle, fit_strat: FitStrategy) -> Rectangle {
        self.fit_into_ex(frame, fit_strat, false)
    }
    #[must_use]
    /// Shrinks (or grows) and moves the rectangle to fit within the given frame, without changing proportions
    fn fit_into(&self, frame: &Rectangle, fit_strat: FitStrategy) -> Rectangle {
        self.fit_into_ex(frame, fit_strat, true)
    }
}

pub trait JmrVector {
    fn distance_2(&self, other: &Vector) -> f32;
}

impl JmrRectangle for Rectangle {
    fn shrink_to_center(&self, shrink_to_center: f32) -> Rectangle {
        Rectangle::new_sized(self.size() * shrink_to_center).with_center(self.center())
    }
    /// Padds constant pixels around the rectangle
    fn padded(&self, padding: f32) -> Rectangle {
        Rectangle::new_sized(self.size() - (2.0 * padding, 2.0 * padding).into())
            .with_center(self.center())
    }
    /// Shrinks and moves the rectangle to fit within the given frame, without changing proportions
    fn fit_into_ex(
        mut self,
        frame: &Rectangle,
        fit_strat: FitStrategy,
        allow_grow: bool,
    ) -> Rectangle {
        let stretch_factor = (frame.width() / self.width()).min(frame.height() / self.height());
        if allow_grow || stretch_factor < 1.0 {
            self.size *= stretch_factor;
        }
        match fit_strat {
            FitStrategy::TopLeft => self.pos = frame.pos,
            FitStrategy::Center => {
                self.pos = frame.pos;
                self.pos = frame.pos + frame.center() - self.center()
            }
        }
        self
    }
    /// Finds the largest square that fits into the given rectangle
    fn fit_square(&self, fit_strat: FitStrategy) -> Rectangle {
        let s = self.width().min(self.height());
        let mut rect = Rectangle::new(self.pos, (s, s));
        match fit_strat {
            FitStrategy::Center => {
                rect = rect.translate(((self.width() - rect.width()) / 2.0, 0.0));
                rect = rect.translate((0.0, (self.height() - rect.height()) / 2.0));
            }
            FitStrategy::TopLeft => {}
        }
        rect
    }
    fn grid(&self, cols: usize, rows: usize) -> Grid {
        let dx = self.width() / cols as f32;
        let dy = self.height() / rows as f32;
        Grid {
            base: Rectangle::new(self.pos, (dx, dy)),
            i: 0,
            x: cols,
            y: rows,
        }
    }
    fn cut_horizontal(&self, h: f32) -> (Rectangle, Rectangle) {
        let mut top = self.clone();
        top.size.y = h;
        let mut bottom = self.clone();
        bottom.size.y -= h;
        bottom.pos.y += h;
        (top, bottom)
    }
    fn cut_vertical(&self, w: f32) -> (Rectangle, Rectangle) {
        let mut left = self.clone();
        left.size.x = w;
        let mut right = self.clone();
        right.size.x -= w;
        right.pos.x += w;
        (left, right)
    }
}

impl JmrVector for Vector {
    fn distance_2(&self, other: &Vector) -> f32 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        x * x + y * y
    }
}