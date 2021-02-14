#[derive(Copy, Clone, Debug)]
pub struct HoboLevel(usize);
#[derive(Copy, Clone, Debug)]
pub enum HoboType {
    Yellow,
    Camo,
    White,
    Prophet,
    /// Pick one of Yellow/Camo/White
    DefaultRandom,
}

#[derive(Copy, Clone, Debug)]
pub struct VisitorDefinition {
    pub typ: HoboType,
    pub level: HoboLevel,
    pub hp: Option<u16>,
    pub hurried: bool,
}

impl VisitorDefinition {
    pub const fn new(typ: HoboType, level: HoboLevel, hurried: bool) -> Self {
        Self {
            typ,
            hurried,
            hp: None,
            level,
        }
    }
    pub const fn new_fixed_hp(typ: HoboType, hurried: bool, hp: u16) -> Self {
        Self {
            typ,
            hurried,
            hp: Some(hp),
            level: HoboLevel::zero(),
        }
    }
}

impl HoboLevel {
    pub const fn zero() -> Self {
        HoboLevel(0)
    }
    pub fn anarchist(player_karma: i64) -> Self {
        match player_karma {
            0..=9 => HoboLevel(0),
            10..=99 => HoboLevel(1),
            100..=899 => HoboLevel(1 + player_karma as usize / 100),
            _ => HoboLevel(10),
        }
    }
    /// Right-exclusive range
    pub fn hurried_anarchist_hp_range(&self) -> (i64, i64) {
        // TODO: Balancing
        match self.0 {
            0 => (1, 2),
            1 => (1, 3),
            2 => (1, 4),
            3 => (2, 5),
            4 => (3, 8),
            5 => (4, 9),
            6 => (6, 13),
            7 => (10, 20),
            8 => (20, 40),
            9 => (30, 60),
            _ => (50, 100),
        }
    }
    pub fn unhurried_anarchist_hp(&self) -> i64 {
        self.hurried_anarchist_hp_range().1
    }
}
