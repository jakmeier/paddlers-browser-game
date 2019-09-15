use super::{
    animation::{AnimatedObject, AnimatedObjectDef, AnimationVariantDef},
    paths::{ANIMATION_DEFS, SPRITE_PATHS},
    Sprites,
};
use crate::game::Game;
use quicksilver::prelude::*;

pub struct Preloading {
    images: Vec<Asset<Image>>,
}

impl Preloading {
    pub fn new() -> Self {
        let images = start_loading_sprites();
        Preloading { images }
    }
    pub fn progress(&mut self) -> f32 {
        let total = self.images.len() as f32;
        let count = self
            .images
            .iter_mut()
            .map(|asset| {
                let mut helper = false;
                asset
                    .execute(|_| {
                        helper = true;
                        Ok(())
                    })
                    .unwrap();
                helper
            })
            .filter(|b| *b)
            .count();
        count as f32 / total
    }
    pub fn finalize(self) -> Vec<Image> {
        self.images
            .into_iter()
            .map(|mut asset| {
                let mut helper = None;
                asset
                    .execute(|img| {
                        helper = Some(img.clone());
                        Ok(())
                    })
                    .unwrap();
                helper.unwrap()
            })
            .collect()
    }
}

fn start_loading_sprites() -> Vec<Asset<Image>> {
    let images: Vec<Asset<Image>> = SPRITE_PATHS.iter().map(load_image).collect();
    images
}

pub fn start_loading_animations(images: &Vec<Image>) -> Vec<(Asset<AnimatedObject>, Image)> {
    let animations = ANIMATION_DEFS
        .iter()
        .map(|a| load_animation(a, images))
        .collect();
    animations
}

fn load_image(path: &&'static str) -> Asset<Image> {
    Asset::new(Image::load(*path))
}
fn load_image_from_variant(v: &AnimationVariantDef) -> impl Future<Item = Image, Error = Error> {
    match v {
        AnimationVariantDef::Animated(path) | AnimationVariantDef::Static(path) => {
            Image::load(*path)
        }
    }
}
fn load_animation(
    def: &'static AnimatedObjectDef,
    images: &Vec<Image>,
) -> (Asset<AnimatedObject>, Image) {
    let futures = join_all(vec![
        load_image_from_variant(&def.up),
        load_image_from_variant(&def.left),
        load_image_from_variant(&def.down),
        load_image_from_variant(&def.standing),
    ]);
    let cols = def.cols as u32;
    let rows = def.rows as u32;
    let obj = futures.map(move |res| {
        let mut iter = res.into_iter();
        AnimatedObject::walking(
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            cols,
            rows,
            iter.next().unwrap(),
        )
    });
    (
        Asset::new(obj),
        images[def.alternative.index_in_vector()].clone(),
    )
}

impl Game<'static, 'static> {
    pub fn update_preloading(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }
    pub fn draw_preloading(&mut self, window: &mut Window) -> Result<()> {
        let progress = self.preload.as_mut().unwrap().progress();
        if progress < 1.0 {
            // TODO: Display loading progress
            println!("Loaded {:.3}%", progress * 100.0);
            return Ok(());
        }
        println!("Finalizing");
        let images = self.preload.take().unwrap().finalize();
        println!("Finalized");
        self.sprites = Some(Sprites::new(images));
        println!("Finalized II");
        Ok(())
    }
}
