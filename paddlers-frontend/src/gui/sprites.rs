pub mod animation;
pub mod paths;

use quicksilver::prelude::*;
use quicksilver::graphics::Image;
use crate::gui::utils::*;
use animation::AnimatedObject;
use crate::init::loading::start_loading_animations;

/// Manager of all sprites.
/// Cannot easily be in a component because Image is thread local.
pub struct Sprites {
    img: Vec<Image>,
    animations: Vec<(Asset<AnimatedObject>, Image)>,
}

impl Sprites {
    pub fn new(images: Vec<Image>) -> Self {
        let future_animations = start_loading_animations(&images);
        Sprites {
            img: images,
            animations: future_animations,
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// An instance of a SpriteIndex is a key for a specific sprite (Something that can be drawn uniquely)
pub enum SpriteIndex {
    // Multi-sprite images 
    Simple(SingleSprite),
    Directed(DirectedSprite, Direction),
    Animated(AnimatedSprite, Direction, u32),
}

/// An instance of a SpriteSet summarizes one or many sprites that show 
/// the same object in different states / from different angles
#[derive(Debug, Clone, Copy)]
pub enum SpriteSet {
    Simple(SingleSprite),
    #[allow(dead_code)]
    Directed(DirectedSprite),
    Animated(AnimatedSprite),
}

impl SpriteSet {
    pub fn default(&self) -> SpriteIndex {
        match self {
            SpriteSet::Simple(i) => SpriteIndex::Simple(*i),
            SpriteSet::Directed(i) => SpriteIndex::Directed(*i, Direction::Undirected),
            SpriteSet::Animated(i) => SpriteIndex::Animated(*i, Direction::Undirected, 0),
        }
    }
    #[allow(dead_code)]
    pub fn directed(&self, d: &Direction) -> (SpriteIndex, Transform) {
        let i = match self {
            SpriteSet::Simple(i) => SpriteIndex::Simple(*i),
            SpriteSet::Directed(i) => SpriteIndex::Directed(*i, *d),
            SpriteSet::Animated(i) => SpriteIndex::Animated(*i, *d, 0),
        };
        let t = match d {
            Direction::East => { horizontal_flip() },
            _ => { Transform::IDENTITY },
        };
        (i,t)
    }
    pub fn animated(&self, d: &Direction, animation_frame: u32) -> (SpriteIndex, Transform) {
        let i = match self {
            SpriteSet::Simple(i) => SpriteIndex::Simple(*i),
            SpriteSet::Directed(i) => SpriteIndex::Directed(*i, *d),
            SpriteSet::Animated(i) => SpriteIndex::Animated(*i, *d, animation_frame),
        };
        let t = match d {
            Direction::East => { horizontal_flip() },
            _ => { Transform::IDENTITY },
        };
        (i,t)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SingleSprite {
    Grass,
    GrassTop,
    GrassBot,
    Water,
    Duck,
    RedFlowers,
    BlueFlowers,
    Feathers,
    Sticks,
    Logs,
    Heart,
    Ambience,
    Tree,
    Sapling, 
    YoungTree,
    CamoDuck,
    WhiteDuck,
    BundlingStation,
    SawMill,
    MapButton,
    MapButtonHov,
    TownButton,
    TownButtonHov,
    Shack,
    DuckSteps,
    Roger,
    NewOrder,
    WelcomeAbility,
    #[allow(dead_code)]
    FrameBlue1,
    #[allow(dead_code)]
    FrameBlue2,
    #[allow(dead_code)]
    FrameBlue3,
    #[allow(dead_code)]
    FrameGreen1,
    #[allow(dead_code)]
    FrameGreen2,
    #[allow(dead_code)]
    FrameGreen3,
    PresentA,
    PresentB,
    DuckHappy,
    CamoDuckHappy,
    WhiteDuckHappy,
    Temple,
    Karma,
    Prophet,
}

#[derive(Debug, Clone, Copy)]
pub enum DirectedSprite {
    VerticalLeaves,
}

#[derive(Debug, Clone, Copy)]
pub enum AnimatedSprite {
    Roger,
}

impl Sprites {
    pub fn index(&mut self, index: SpriteIndex) -> Image {
        match index {
            SpriteIndex::Simple(j) => {
                let i = j.index_in_vector();
                self.img[i].clone()
            },
            SpriteIndex::Directed(j,d) => { 
                let i = j.index_in_vector(d);
                self.img[i].clone()
            },
            SpriteIndex::Animated(j,d,a) => {
                let i = j.index_in_vector();
                let animations = &mut self.animations;

                // TODO: Find better way to read return value from Asset execution
                let mut result =  animations[i].1.clone();
                animations[i].0.execute(
                    |v| { result = v.sprite(d,a); Ok(()) },
                ).unwrap();
                result
            }
        }
    }
}

pub trait WithSprite {
    fn sprite(&self) -> SpriteSet;
}

use paddlers_shared_lib::models::BuildingType;
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