//! Provides the tools to connect UI components nicely 
//! E.g. drawing fancy borders while filling exactly the 
//! left over space for any given screen setting
//! 
//! Much of this is WIP and specifically tailored towards one use-case.
//! Hence, do not expect this to be reusable in the current form. 
//! (It might become reusable in the future, though!)

use quicksilver::prelude::*;
use crate::gui::{
    z::*,
    utils::*,
    sprites::*,
};

pub fn draw_leaf_border(window: &mut Window, sprites: &mut Asset<Sprites>, area: &Rectangle) {
    sprites.execute( |sprites| {
        let lv = DirectedSprite::VerticalLeaves;
        let top = &sprites[SpriteIndex::Directed(lv, Direction::North)];
        let mid = &sprites[SpriteIndex::Directed(lv, Direction::Undirected)];
        let bot = &sprites[SpriteIndex::Directed(lv, Direction::South)];

        let w = top.area().width();
        let start = area.pos.translate((-w*0.75,0));
        draw_column_texture(window, top, mid, bot, start, area.y() + area.height());
        draw_column_texture(window, top, mid, bot, start.translate((area.width(), 0)), area.y() + area.height());

        let leaves = &sprites[SpriteIndex::Directed(lv, Direction::East)];
        let h = leaves.area().height();
        let start = area.pos.translate((0, -h/2.0));
        fill_row_with_img(window, leaves, start, area.x() + area.width());
        fill_row_with_img(window, leaves, start.translate((0,area.height())), area.x() + area.width());

        Ok(())
    }).unwrap();
}

fn draw_column_texture(window: &mut Window, top: &Image, mid: &Image, bot: &Image, start: Vector, end: f32) {
    let mut stamp = top.area();
    stamp.pos = start;
    window.draw_ex(
        &stamp,
        Img(top),
        Transform::IDENTITY, 
        Z_UI_BORDERS,
    );
    stamp.pos.y += stamp.height();
    stamp.size.y = mid.area().height();
    while stamp.y() + stamp.height() < end {
        window.draw_ex(
            &stamp,
            Img(mid),
            Transform::IDENTITY, 
            Z_UI_BORDERS,
        );
        stamp.pos.y += stamp.height();
    }
    stamp.size.y = bot.area().height();
    window.draw_ex(
        &stamp,
        Img(bot),
        Transform::IDENTITY, 
        Z_UI_BORDERS,
    );
}

fn fill_row_with_img(window: &mut Window, img: &Image, start: Vector, end: f32) {
    let mut stamp = img.area();
    stamp.pos = start;
    while stamp.x() < end {
        window.draw_ex(
            &stamp,
            Img(img),
            Transform::IDENTITY, 
            Z_UI_BORDERS - 1,
        );
        stamp.pos.x += stamp.width() * 0.9;
    }
}