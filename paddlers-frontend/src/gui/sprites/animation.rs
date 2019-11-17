use super::SingleSprite;
use crate::gui::utils::Direction;
use quicksilver::prelude::*;

/// Stores the sprites of an animated object.
/// Provides functions to render the object in different contexts.
/// The time dependent state of the object is not stored in this struct.
pub struct AnimatedObject {
    up: AnimationVariant,
    left: AnimationVariant,
    down: AnimationVariant,
    standing: AnimationVariant,
}
enum AnimationVariant {
    Animated(Animation),
    Static(Image),
}
/// Holds data for a sprite-sheet.
pub struct Animation {
    sprite_sheet: Image,
    cols: u32,
    rows: u32,
}

/// Defines a sprite sheet to be downloaded as AnimatedObject
pub struct AnimatedObjectDef {
    pub up: AnimationVariantDef,
    pub left: AnimationVariantDef,
    pub down: AnimationVariantDef,
    pub standing: AnimationVariantDef,
    pub cols: u8,
    pub rows: u8,
    /// Is displayed until animation has been downloaded
    pub alternative: SingleSprite,
}
pub enum AnimationVariantDef {
    Animated(&'static str),
    Static(&'static str),
}

impl AnimatedObject {
    /// Animated while walking, static image while standing.
    /// Provide sprite sheets in first 3 parameters with shared frame numbers in 4th and 5th parameter, and single image in last parameter.
    /// Right walking will be mirrored from left walking.
    pub fn walking(
        up: Image,
        down: Image,
        left: Image,
        cols: u32,
        rows: u32,
        stand: Image,
    ) -> Self {
        AnimatedObject {
            up: AnimationVariant::Animated(Animation::new(up, cols, rows)),
            left: AnimationVariant::Animated(Animation::new(down, cols, rows)),
            down: AnimationVariant::Animated(Animation::new(left, cols, rows)),
            standing: AnimationVariant::Static(stand),
        }
    }
    pub fn sprite(&self, d: Direction, frame: u32) -> Image {
        let animation = match d {
            Direction::North => &self.up,
            Direction::East => &self.left,
            Direction::South => &self.down,
            Direction::West => &self.left, // will be flipped when drawing
            Direction::Undirected => &self.standing,
        };
        match animation {
            AnimationVariant::Animated(a) => a.sprite(frame),
            AnimationVariant::Static(i) => i.clone(),
        }
    }
}
impl Animation {
    fn new(sprite_sheet: Image, cols: u32, rows: u32) -> Self {
        Animation {
            sprite_sheet,
            cols,
            rows,
        }
    }
    fn sprite(&self, frame: u32) -> Image {
        let base = self.sprite_sheet.area();

        let n = self.cols * self.rows;
        let i = frame % n;

        let w = base.width() / self.cols as f32;
        let h = base.height() / self.rows as f32;

        let x = w * (i % self.cols) as f32;
        let y = h * (i / self.cols) as f32;

        let region = Rectangle::new((x, y), (w, h));
        self.sprite_sheet.subimage(region)
    }
}
