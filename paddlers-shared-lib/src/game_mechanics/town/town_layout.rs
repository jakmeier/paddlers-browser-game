use super::*;

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

pub type TownLayoutIndex = (i32, i32);
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

static BASIC_PATH: [TownLayoutIndex; 11] = [
    (TOWN_X as i32, TOWN_LANE_Y as i32),
    (8, TOWN_LANE_Y as i32),
    (7, TOWN_LANE_Y as i32),
    (6, TOWN_LANE_Y as i32),
    (5, TOWN_LANE_Y as i32),
    (TOWN_RESTING_X as i32, TOWN_LANE_Y as i32),
    (3, TOWN_LANE_Y as i32),
    (2, TOWN_LANE_Y as i32),
    (1, TOWN_LANE_Y as i32),
    (0, TOWN_LANE_Y as i32),
    (-1, TOWN_LANE_Y as i32),
];

impl ITownLayout for TownLayout {
    type Index = TownLayoutIndex;
    fn path_to_rest_place(&self) -> &'static [Self::Index] {
        match self {
            Self::Basic => &BASIC_PATH[0..6],
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
