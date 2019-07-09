use quicksilver::prelude::*;
use quicksilver::graphics::Color;
use specs::prelude::*;
use crate::game::{
    Game,
    sprites::{SpriteIndex, Sprites},
    movement::Position,
    input::MenuBoxData,
};


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub kind: RenderType,
}
#[derive(Debug)]
pub enum RenderType {
    StaticImage(SpriteIndex)
}  

pub const GREY: Color =    Color { r: 0.75, g: 0.75, b: 0.75, a: 1.0 };

impl Game<'_, '_> {
    pub fn render_entities(&mut self, window: &mut Window) -> Result<()> {
        let world = &self.world;
        let pos_store = world.read_storage::<Position>();
        let rend_store = world.read_storage::<Renderable>();
        let sprites = &mut self.sprites;
        for (pos, r) in (&pos_store, &rend_store).join() {
            match r.kind {
                RenderType::StaticImage(i) => {
                    draw_static_image(sprites, window, &pos.area, i, 0, FitStrategy::TopLeft)?;
                },
            }
        }
        Ok(())
    }
    pub fn render_menu_box(&mut self, window: &mut Window) -> Result<()> {
        let data = self.world.read_resource::<MenuBoxData>();
        let entity = (*data).selected_entity;

        // Menu Box Background
        window.draw_ex(
            &data.area,
            Col(GREY),
            Transform::rotate(0), 
            10
        );

        // Image
        let mut img_bg_area = data.area.clone();
        img_bg_area.size.y = img_bg_area.height() / 3.0;
        let img_bg_area = img_bg_area.fit_square(FitStrategy::Center).padded(0.8);
        let img_area = img_bg_area.padded(0.8);
        match entity {
            Some(id) => {
                // Background of image
                draw_static_image(&mut self.sprites, window, &img_bg_area, SpriteIndex::Water, 15, FitStrategy::Center)?;

                // Image
                let e = self.world.entities().entity(id);
                let r = self.world.read_storage::<Renderable>();
                let sprites = &mut self.sprites;
                let rd = r.get(e).expect("Selected item should have Renderable component");
                match rd.kind {
                    RenderType::StaticImage(i) => {
                        draw_static_image(sprites, window, &img_area, i, 20, FitStrategy::Center)?;
                    },
                }
            },
            None => {

            },
        }
        Ok(())
    }

}

#[derive(Copy, Clone, Debug)]
enum FitStrategy {
    TopLeft,
    Center
}

fn draw_static_image(asset: &mut Asset<Sprites>, window: &mut Window, max_area: &Rectangle, i: SpriteIndex, z: i32, fit_strat: FitStrategy) -> Result<()> {
    asset.execute( |sprites| {
        let img = &sprites[i];
        let mut area = *max_area;
        let img_slope = img.area().height() / img.area().width();
        if img_slope < area.height() / area.width() {
            // high image
            area.size.y = area.width() * img_slope;
            match fit_strat {
                FitStrategy::Center => {
                    area = area.translate((0,(max_area.height() - area.height())/2.0));
                },
                FitStrategy::TopLeft => {},
            }
        } else {
            area.size.x = area.height() / img_slope;
            match fit_strat {
                FitStrategy::Center => {
                    area = area.translate(((max_area.width() - area.width())/2.0, 0.0));
                },
                FitStrategy::TopLeft => {},
            }
        }
        
        window.draw_ex(
            &area,
            Img(img),
            Transform::rotate(0), 
            z
        );
        Ok(())
    })
}

trait JmrRectangle {
    fn padded(&self, padding_factor: f32) -> Rectangle;
    fn fit_square(&self, fit_strat: FitStrategy) -> Rectangle;
}

impl JmrRectangle for Rectangle{
    fn padded(&self, padding_factor: f32) -> Rectangle {
        Rectangle::new_sized(self.size() * padding_factor)
            .with_center(self.center())
    }
    fn fit_square(&self, fit_strat: FitStrategy) -> Rectangle {
        let s = self.width().min(self.height());
        let mut rect = Rectangle::new(self.pos, (s,s));
        match fit_strat {
            FitStrategy::Center => {
                rect = rect.translate(((self.width() - rect.width())/2.0, 0.0));
                rect = rect.translate((0.0, (self.height() - rect.height())/2.0));
            },
            FitStrategy::TopLeft => {},
        }
        rect
    }
}
