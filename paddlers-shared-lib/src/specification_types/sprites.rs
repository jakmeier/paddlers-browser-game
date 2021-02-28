pub mod sprite_paths;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
/// An instance of a SpriteIndex is a key for a specific sprite (Something that maps uniquely to a quicksilver Image)
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

#[derive(Debug, Clone, Copy, Deserialize)]
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
    Population,
    Prophet,
    AttacksButton,
    AttacksButtonHov,
    LeaderboardButton,
    LeaderboardButtonHov,
    RogerLarge,
    RogerLargeCelebrating,
    RogerLargeObedient,
    RogerLargeSad,
    RogerLargeAstonished,
    Letters,
    DuckShapes,
    SingleNest,
    TripleNest,
    SittingYellowDuck,
    Stone1,
    Stone2,
    PerkConversion,
    PerkInvitation,
    PerkNestBuilding,
    PerkTripleNestBuilding,
    ReligionDroplets,
    SingleDuckShape,
    SingleDuckBackgroundShape,
    VisitorGateSymbol,
    Plus,
    Duties,
    Letter,
    LetterHov,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum DirectedSprite {
    VerticalLeaves,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum AnimatedSprite {
    Roger,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Direction {
    Undirected,
    North,
    East,
    South,
    West,
}
