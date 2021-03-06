use super::*;

pub const SPRITE_PATHS_NUM: usize = 75;
pub const SPRITE_PATHS: [&'static str; SPRITE_PATHS_NUM] = [
    "textures/grass.png", // 0
    "textures/water.png",
    "ducks/duck_sad_yellow.png",
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
    "ducks/duck_sad_camo.png",
    "ducks/duck_sad_white.png",
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
    "gui/abilities/work.png", // 30
    "gui/abilities/welcome.png",
    "gui/abilities/blue_frame_1.png",
    "gui/abilities/blue_frame_2.png",
    "gui/abilities/blue_frame_3.png",
    "gui/abilities/green_frame_1.png",
    "gui/abilities/green_frame_2.png",
    "gui/abilities/green_frame_3.png",
    "buildings/red_present.png",
    "buildings/orange_present.png",
    "ducks/duck_happy_yellow.png", // 40
    "ducks/duck_happy_camo.png",
    "ducks/duck_happy_white.png",
    "buildings/temple.png",
    "resources/karma.png",
    "ducks/prophet_swimming.png",
    "gui/attacks_button.png",
    "gui/attacks_button_hov.png",
    "gui/leaderboard_button.png",
    "gui/leaderboard_button_hov.png",
    "ducks/roger_large.png", // 50
    "ducks/roger_celebrating.png",
    "ducks/roger_obedient.png",
    "ducks/roger_sad.png",
    "ducks/roger_astonished.png",
    "gui/letters.png",
    "gui/duck_shapes.png",
    "buildings/nest.png",
    "buildings/nests.png",
    "ducks/sitting_duck.png",
    "stone_1.png", // 60
    "stone_2.png",
    "perks/conversion.png",
    "perks/invitation.png",
    "perks/nest_building.png",
    "perks/triple_nest_building.png",
    "religion.png",
    "gui/duck_shape.png",
    "gui/duck_background_shape.png",
    "gui/visitor_gate.png",
    "gui/plus.png", // 70
    "resources/population.png",
    "gui/duties.png",
    "gui/letter.png",
    "gui/letter_hov.png",
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
            SingleSprite::NewOrder => 30,
            SingleSprite::WelcomeAbility => 31,
            SingleSprite::FrameBlue1 => 32,
            SingleSprite::FrameBlue2 => 33,
            SingleSprite::FrameBlue3 => 34,
            SingleSprite::FrameGreen1 => 35,
            SingleSprite::FrameGreen2 => 36,
            SingleSprite::FrameGreen3 => 37,
            SingleSprite::PresentA => 38,
            SingleSprite::PresentB => 39,
            SingleSprite::DuckHappy => 40,
            SingleSprite::CamoDuckHappy => 41,
            SingleSprite::WhiteDuckHappy => 42,
            SingleSprite::Temple => 43,
            SingleSprite::Karma => 44,
            SingleSprite::Prophet => 45,
            SingleSprite::AttacksButton => 46,
            SingleSprite::AttacksButtonHov => 47,
            SingleSprite::LeaderboardButton => 48,
            SingleSprite::LeaderboardButtonHov => 49,
            SingleSprite::RogerLarge => 50,
            SingleSprite::RogerLargeCelebrating => 51,
            SingleSprite::RogerLargeObedient => 52,
            SingleSprite::RogerLargeSad => 53,
            SingleSprite::RogerLargeAstonished => 54,
            SingleSprite::Letters => 55,
            SingleSprite::DuckShapes => 56,
            SingleSprite::SingleNest => 57,
            SingleSprite::TripleNest => 58,
            SingleSprite::SittingYellowDuck => 59,
            SingleSprite::Stone1 => 60,
            SingleSprite::Stone2 => 61,
            SingleSprite::PerkConversion => 62,
            SingleSprite::PerkInvitation => 63,
            SingleSprite::PerkNestBuilding => 64,
            SingleSprite::PerkTripleNestBuilding => 65,
            SingleSprite::ReligionDroplets => 66,
            SingleSprite::SingleDuckShape => 67,
            SingleSprite::SingleDuckBackgroundShape => 68,
            SingleSprite::VisitorGateSymbol => 69,
            SingleSprite::Plus => 70,
            SingleSprite::Population => 71,
            SingleSprite::Duties => 72,
            SingleSprite::Letter => 73,
            SingleSprite::LetterHov => 74,
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

impl AnimatedSprite {
    pub fn index_in_vector(&self) -> usize {
        match self {
            AnimatedSprite::Roger => 0,
        }
    }
}
