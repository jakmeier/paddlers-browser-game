use quicksilver::prelude::*;

pub struct Grid {
    pub base: Rectangle,
    pub i: usize,
    pub x: usize,
    pub y: usize,
}
impl Iterator for Grid {
    type Item = Rectangle;
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x;
        let y = self.y;
        let i = self.i;
        if i < x * y {
            let w = self.base.width();
            let h = self.base.height();
            let mut r = self.base.clone();
            r.pos.x = self.base.pos.x + w * (i % x) as f32;
            r.pos.y = self.base.pos.y + h * (i / x) as f32;
            self.i += 1;
            Some(r)
        } else {
            None
        }
    }
}
