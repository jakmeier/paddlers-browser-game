//! Provides the tools to connect UI components nicely
//! E.g. drawing fancy borders while filling exactly the
//! left over space for any given screen setting
//!
//! Much of this is WIP and specifically tailored towards one use-case.
//! Hence, do not expect this to be reusable in the current form.
//! (It might become reusable in the future, though!)

use crate::gui::{sprites::*, utils::*, z::*};
use paddle::*;
use paddle::{quicksilver_compat::*, DisplayArea};

pub fn draw_leaf_border(
    window: &mut DisplayArea,
    sprites: &mut Sprites,
    area: &Rectangle,
    leaf_w: f32,
    leaf_h: f32,
) {
    let lv = DirectedSprite::VerticalLeaves;
    let top = sprites
        .index(SpriteIndex::Directed(lv, Direction::North))
        .clone();
    let mid = sprites
        .index(SpriteIndex::Directed(lv, Direction::Undirected))
        .clone();
    let bot = sprites
        .index(SpriteIndex::Directed(lv, Direction::South))
        .clone();

    let w = leaf_w;
    let start = area.pos.translate((-w * 0.75, 0));
    draw_column_texture(window, &top, &mid, &bot, start, area.y() + area.height(), w);
    draw_column_texture(
        window,
        &top,
        &mid,
        &bot,
        start.translate((area.width(), 0)),
        area.y() + area.height(),
        w,
    );

    let leaves = &sprites.index(SpriteIndex::Directed(lv, Direction::East));
    let h = leaf_h;
    let start = area.pos.translate((0, -h / 2.0));
    fill_row_with_img(window, leaves, start, area.x() + area.width(), h);
    fill_row_with_img(
        window,
        leaves,
        start.translate((0, area.height())),
        area.x() + area.width(),
        h,
    );
}

fn draw_column_texture(
    window: &mut DisplayArea,
    top: &Image,
    mid: &Image,
    bot: &Image,
    start: Vector,
    end: f32,
    w: f32,
) {
    let mut stamp = Rectangle::new(start, top.natural_size());
    let factor = w / stamp.width();
    stamp.size = stamp.size * factor;
    window.draw_ex(&stamp, top, Transform::IDENTITY, Z_UI_BORDERS);
    stamp.pos.y += stamp.height();
    stamp.size = mid.natural_size() * factor;
    while stamp.y() + stamp.height() < end {
        window.draw_ex(&stamp, mid, Transform::IDENTITY, Z_UI_BORDERS);
        stamp.pos.y += stamp.height();
    }
    stamp.size = bot.natural_size() * factor;
    window.draw_ex(&stamp, bot, Transform::IDENTITY, Z_UI_BORDERS);
}

fn fill_row_with_img(window: &mut DisplayArea, img: &Image, start: Vector, end: f32, h: f32) {
    let mut stamp = Rectangle::new(start, img.natural_size());
    let factor = h / stamp.height();
    stamp.size = stamp.size * factor;
    while stamp.x() < end {
        window.draw_ex(&stamp, img, Transform::IDENTITY, Z_UI_BORDERS - 1);
        stamp.pos.x += stamp.width() * 0.9;
    }
}

pub fn draw_duck_step_line(
    window: &mut DisplayArea,
    sprites: &mut Sprites,
    start: Vector,
    end: f32,
    h: f32,
) {
    let img = &sprites.index(SpriteIndex::Simple(SingleSprite::DuckSteps));
    fill_row_with_img(window, img, start, end, h);
}
