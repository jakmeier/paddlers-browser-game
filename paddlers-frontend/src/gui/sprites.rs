pub mod animation;
pub mod paths;

use super::z::*;
use crate::gui::utils::*;
use animation::AnimatedObject;
use mogwai::prelude::*;
use paddle::*;
pub use paddlers_shared_lib::specification_types::*;

/// Manager of all sprites.
/// Cannot easily be in a component because Image is thread local.
pub struct Sprites {
    img: Vec<Image>,
    animations: Vec<AnimatedObject>,
}

impl Sprites {
    pub fn new(images: Vec<Image>, animations: Vec<AnimatedObject>) -> Self {
        Sprites {
            img: images,
            animations,
        }
    }
    pub fn index(&self, index: SpriteIndex) -> Image {
        match index {
            SpriteIndex::Simple(j) => {
                let i = j.index_in_vector();
                self.img[i].clone()
            }
            SpriteIndex::Directed(j, d) => {
                let i = j.index_in_vector(d);
                self.img[i].clone()
            }
            SpriteIndex::Animated(j, d, a) => {
                let animations = &self.animations;
                let i = j.index_in_vector();
                animations[i].sprite(d, a)
            }
        }
    }
    // pub fn new_image_node(img: SpriteIndex) -> HtmlImageElement {
    //     let node = HtmlImageElement::new().unwrap();
    //     let i = match img {
    //         SpriteIndex::Simple(x) => x.index_in_vector(),
    //         _ => unimplemented!(),
    //     };
    //     let img_src = paths::SPRITE_PATHS[i];
    //     node.set_src(img_src);
    //     node
    // }
    #[allow(unused_braces)]
    pub fn new_image_node_builder(img: SpriteIndex) -> ViewBuilder<HtmlElement> {
        let i = match img {
            SpriteIndex::Simple(x) => x.index_in_vector(),
            _ => unimplemented!(),
        };
        let img_src = sprite_paths::SPRITE_PATHS[i];

        builder!(
            <img src={img_src} />
        )
    }
}

/// Single sprite representation of an object
pub trait WithSprite {
    fn sprite(&self) -> SpriteSet;
}
/// Fully rendered represenation of an object
pub trait WithRenderVariant {
    fn render_variant(&self) -> RenderVariant;
    fn on_selection_render_variant(&self) -> Option<RenderVariant>;
}

use paddlers_shared_lib::{civilization::CivilizationPerk, models::BuildingType};
impl WithRenderVariant for BuildingType {
    fn render_variant(&self) -> RenderVariant {
        match self {
            BuildingType::Watergate => RenderVariant::ImgCollection(
                ImageCollection::new(
                    (1.0, 1.0),
                    vec![
                        SubImg::new(SingleSprite::Stone1, (0.25, 0), (0.25, 0.25), 0),
                        SubImg::new(
                            SingleSprite::Stone2,
                            (0.25, 0.75),
                            (0.25, 0.25),
                            Z_VISITOR - Z_BUILDINGS + 1,
                        ),
                    ],
                )
                .with_background(SingleSprite::Water),
            ),
            _ => RenderVariant::ImgWithImgBackground(self.sprite(), SingleSprite::Grass),
        }
    }

    fn on_selection_render_variant(&self) -> Option<RenderVariant> {
        match self {
            BuildingType::Watergate => Some(RenderVariant::Img(SpriteSet::Simple(
                SingleSprite::VisitorGateSymbol,
            ))),
            _ => None,
        }
    }
}

impl WithSprite for BuildingType {
    fn sprite(&self) -> SpriteSet {
        match self {
            BuildingType::BlueFlowers => SpriteSet::Simple(SingleSprite::BlueFlowers),
            BuildingType::RedFlowers => SpriteSet::Simple(SingleSprite::RedFlowers),
            BuildingType::Tree => SpriteSet::Simple(SingleSprite::Sapling),
            BuildingType::BundlingStation => SpriteSet::Simple(SingleSprite::BundlingStation),
            BuildingType::SawMill => SpriteSet::Simple(SingleSprite::SawMill),
            BuildingType::PresentA => SpriteSet::Simple(SingleSprite::PresentA),
            BuildingType::PresentB => SpriteSet::Simple(SingleSprite::PresentB),
            BuildingType::Temple => SpriteSet::Simple(SingleSprite::Temple),
            BuildingType::SingleNest => SpriteSet::Simple(SingleSprite::SingleNest),
            BuildingType::TripleNest => SpriteSet::Simple(SingleSprite::TripleNest),
            BuildingType::Watergate => SpriteSet::Simple(SingleSprite::Stone1),
        }
    }
}

use paddlers_shared_lib::models::ResourceType;
impl WithSprite for ResourceType {
    fn sprite(&self) -> SpriteSet {
        match self {
            ResourceType::Feathers => SpriteSet::Simple(SingleSprite::Feathers),
            ResourceType::Sticks => SpriteSet::Simple(SingleSprite::Sticks),
            ResourceType::Logs => SpriteSet::Simple(SingleSprite::Logs),
        }
    }
}

pub fn tree_sprite(score: usize) -> SpriteSet {
    match score {
        s if s <= 2 => SpriteSet::Simple(SingleSprite::Sapling),
        s if s <= 9 => SpriteSet::Simple(SingleSprite::YoungTree),
        _ => SpriteSet::Simple(SingleSprite::Tree),
    }
}

use paddlers_shared_lib::prelude::AbilityType;
impl WithSprite for AbilityType {
    fn sprite(&self) -> SpriteSet {
        match self {
            AbilityType::Work => SpriteSet::Simple(SingleSprite::NewOrder),
            AbilityType::Welcome => SpriteSet::Simple(SingleSprite::WelcomeAbility),
        }
    }
}

use paddlers_shared_lib::prelude::TaskType;

impl WithSprite for TaskType {
    fn sprite(&self) -> SpriteSet {
        match self {
            TaskType::Idle => SpriteSet::Simple(SingleSprite::Roger),
            TaskType::Walk => SpriteSet::Simple(SingleSprite::NewOrder),
            TaskType::Defend => SpriteSet::Simple(SingleSprite::NewOrder),
            TaskType::GatherSticks => SpriteSet::Simple(SingleSprite::BundlingStation),
            TaskType::ChopTree => SpriteSet::Simple(SingleSprite::SawMill),
            TaskType::WelcomeAbility => SpriteSet::Simple(SingleSprite::WelcomeAbility),
            TaskType::CollectReward => SpriteSet::Simple(SingleSprite::PresentA),
        }
    }
}

impl WithSprite for CivilizationPerk {
    fn sprite(&self) -> SpriteSet {
        SpriteSet::Simple(match self {
            CivilizationPerk::NestBuilding => SingleSprite::PerkNestBuilding,
            CivilizationPerk::TripleNestBuilding => SingleSprite::PerkTripleNestBuilding,
            CivilizationPerk::Invitation => SingleSprite::PerkInvitation,
            CivilizationPerk::Conversion => SingleSprite::PerkConversion,
        })
    }
}

pub trait ISpriteIndex {
    fn default(&self) -> SpriteIndex;
    fn directed(&self, d: &Direction) -> (SpriteIndex, Transform);
    fn animated(&self, d: &Direction, animation_frame: u32) -> (SpriteIndex, Transform);
}

impl ISpriteIndex for SpriteSet {
    fn default(&self) -> SpriteIndex {
        match self {
            SpriteSet::Simple(i) => SpriteIndex::Simple(*i),
            SpriteSet::Directed(i) => SpriteIndex::Directed(*i, Direction::Undirected),
            SpriteSet::Animated(i) => SpriteIndex::Animated(*i, Direction::Undirected, 0),
        }
    }
    fn directed(&self, d: &Direction) -> (SpriteIndex, Transform) {
        let i = match self {
            SpriteSet::Simple(i) => SpriteIndex::Simple(*i),
            SpriteSet::Directed(i) => SpriteIndex::Directed(*i, *d),
            SpriteSet::Animated(i) => SpriteIndex::Animated(*i, *d, 0),
        };
        let t = match d {
            Direction::East => Transform::horizontal_flip(),
            _ => Transform::IDENTITY,
        };
        (i, t)
    }
    fn animated(&self, d: &Direction, animation_frame: u32) -> (SpriteIndex, Transform) {
        let i = match self {
            SpriteSet::Simple(i) => SpriteIndex::Simple(*i),
            SpriteSet::Directed(i) => SpriteIndex::Directed(*i, *d),
            SpriteSet::Animated(i) => SpriteIndex::Animated(*i, *d, animation_frame),
        };
        let t = match d {
            Direction::East => Transform::horizontal_flip(),
            _ => Transform::IDENTITY,
        };
        (i, t)
    }
}
