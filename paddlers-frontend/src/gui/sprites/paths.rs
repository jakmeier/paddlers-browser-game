use super::*;
use super::animation::{AnimatedObjectDef,AnimationVariantDef};

pub (super) const SPRITE_PATHS_NUM: usize = 32;
pub (super) const SPRITE_PATHS : [&'static str; SPRITE_PATHS_NUM] = [
    "textures/grass.png",
    "textures/water.png",
    "ducks/yellow_sad.png",
    "plants/red_flowers.png",
    "plants/blue_flowers.png",
    "resources/yellow_feather.png",
    "resources/sticks.png",
    "resources/logs.png",
    "happy.png",
    "ambience.png",
    "plants/tree.png",
    "plants/sapling.png",
    "plants/young_tree.png",
    "ducks/camo_duck_sad.png",
    "ducks/white_duck_sad.png",
    "buildings/bundling_station.png",
    "buildings/saw_mill.png",
    "gui/map_button.png",
    "gui/map_button_hov.png",
    "buildings/shack.png",
    "ducks/roger.png", // 20
    "textures/grass_top.png",
    "textures/grass_bot.png",
    "gui/leaves/50px_bot.png",
    "gui/leaves/50px_mid.png",
    "gui/leaves/50px_top.png",
    "gui/leaves/leaves.png",
    "gui/town_button.png",
    "gui/town_button_hov.png",
    "gui/steps.png",
    "gui/abilities/walk.png",
    "gui/abilities/welcome.png",
];

impl SingleSprite {
    pub fn index_in_vector(&self) -> usize {
        match self {
            SingleSprite::Grass => 0,
            SingleSprite::GrassTop => 21,
            SingleSprite::GrassBot => 22,
            SingleSprite::Water => 1,
            SingleSprite::Duck => 2,
            SingleSprite::RedFlowers => 3,
            SingleSprite::BlueFlowers => 4,
            SingleSprite::Feathers => 5,
            SingleSprite::Sticks => 6,
            SingleSprite::Logs => 7,
            SingleSprite::Heart => 8,
            SingleSprite::Ambience => 9,
            SingleSprite::Tree => 10,
            SingleSprite::Sapling => 11,
            SingleSprite::YoungTree => 12,
            SingleSprite::CamoDuck => 13,
            SingleSprite::WhiteDuck => 14,
            SingleSprite::BundlingStation => 15,
            SingleSprite::SawMill => 16,
            SingleSprite::MapButton => 17,
            SingleSprite::MapButtonHov => 18,
            SingleSprite::Shack => 19,
            SingleSprite::TownButton => 27,
            SingleSprite::TownButtonHov => 28,
            SingleSprite::DuckSteps => 29,
            SingleSprite::Roger => 20,
            SingleSprite::WalkAbility => 30,
            SingleSprite::WelcomeAbility => 31,
        }
    }
}

impl DirectedSprite {
    pub fn index_in_vector(&self, d: Direction) -> usize {
        match (self, d) {
            (DirectedSprite::VerticalLeaves, Direction::South) => 23,
            (DirectedSprite::VerticalLeaves, Direction::Undirected) => 24,
            (DirectedSprite::VerticalLeaves, Direction::North) => 25,
            (DirectedSprite::VerticalLeaves, _) => 26,
        }
    }
}

pub (super) const ANIMATION_NUM: usize = 1;
pub (super) const ANIMATION_DEFS : [AnimatedObjectDef; ANIMATION_NUM] = [
    AnimatedObjectDef {
        up: AnimationVariantDef::Animated("ducks/animations/roger_back.png"),
        left: AnimationVariantDef::Animated("ducks/animations/roger_left.png"),
        down: AnimationVariantDef::Animated("ducks/animations/roger_front.png"),
        standing: AnimationVariantDef::Static("ducks/roger.png"),
        cols: 20,
        rows: 5,
        alternative: SingleSprite::Roger,
    },
];

impl AnimatedSprite {
    pub fn index_in_vector(&self) -> usize {
        match self {
            AnimatedSprite::Roger => 0,
        }
    }
}