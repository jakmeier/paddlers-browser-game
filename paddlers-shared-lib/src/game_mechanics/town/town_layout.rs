use super::*;

/// The town layout defines where the lane goes through the tile-grid.
/// Using this abstract view, it's possible to determine in which tile a unit is after walking a certain distance.
/// 
/// A path is defined as list of tiles.
/// At t=0, a unit walking that path is just walking into path_tile[0].
/// It then has to walk a full tile before path_tile[1] is reached.
pub trait ITownLayout {
    type Index: 'static + std::fmt::Debug;
    fn path_to_rest_place(&self) -> &'static [Self::Index];
    fn path_from_rest_place(&self) -> &'static [Self::Index];
    fn path_straight_through(&self) -> &'static [Self::Index];
}
/// Implementing this marker trait, which only involves picking a TownLayout, will auto-implement ITownLayout
pub trait ITownLayoutMarker {
    const LAYOUT: TownLayout;
}

pub enum TownLayout {
    Basic,
}

impl<T: ITownLayoutMarker> ITownLayout for T {
    type Index = TownLayoutIndex;
    #[inline(always)]
    fn path_to_rest_place(&self) -> &'static [Self::Index] {
        Self::LAYOUT.path_to_rest_place()
    }
    #[inline(always)]
    fn path_from_rest_place(&self) -> &'static [Self::Index] {
        Self::LAYOUT.path_from_rest_place()
    }
    #[inline(always)]
    fn path_straight_through(&self) -> &'static [Self::Index] {
        Self::LAYOUT.path_straight_through()
    }
}

static BASIC_PATH: [TownLayoutIndex; 9] = [
    (8, TOWN_LANE_Y),
    (7, TOWN_LANE_Y),
    (6, TOWN_LANE_Y),
    (5, TOWN_LANE_Y),
    (TOWN_RESTING_X, TOWN_LANE_Y),
    (3, TOWN_LANE_Y),
    (2, TOWN_LANE_Y),
    (1, TOWN_LANE_Y),
    (0, TOWN_LANE_Y),
];

impl ITownLayout for TownLayout {
    type Index = TownLayoutIndex;
    fn path_to_rest_place(&self) -> &'static [Self::Index] {
        match self {
            Self::Basic => &BASIC_PATH[0..5],
        }
    }
    fn path_from_rest_place(&self) -> &'static [Self::Index] {
        match self {
            Self::Basic => &BASIC_PATH[5..],
        }
    }
    fn path_straight_through(&self) -> &'static [Self::Index] {
        match self {
            Self::Basic => &BASIC_PATH,
        }
    }
}
