use crate::gui::sprites::{
    animation::{AnimatedObject, AnimatedObjectDef, AnimationVariantDef},
    paths::{ANIMATION_DEFS, SPRITE_PATHS},
    Sprites,
};
use crate::game::Game;
use crate::gui::utils::*;
use quicksilver::prelude::*;

pub struct LoadingState {
    images: Vec<Asset<Image>>,
}

impl LoadingState {
    pub fn new() -> Self {
        let images = start_loading_sprites();
        LoadingState { images }
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
    pub fn update_loading(&mut self, _window: &mut Window) -> Result<()> {
        Ok(())
    }
    pub fn draw_loading(&mut self, window: &mut Window) -> Result<()> {
        let progress = self.preload.as_mut().unwrap().progress();
        if progress < 1.0 {
            self.draw_progress(window, progress);
            return Ok(());
        }
        let images = self.preload.take().unwrap().finalize();
        self.sprites = Some(Sprites::new(images));
        Ok(())
    }
    fn draw_progress(&mut self, window: &mut Window, progress: f32) {
        window.clear(DARK_GREEN).unwrap();
        let size = window.screen_size();
        let (w,h) = (size.x, size.y);
        let area = Rectangle::new((w*0.1,h*0.618),(w*0.8,h*0.2));

        // For now, only images are preloaded and therefore this is done very simply
        let msg = "Downloading images"; 
        draw_progress_bar(window, &mut self.bold_font, area, progress, &msg);
    }
}
