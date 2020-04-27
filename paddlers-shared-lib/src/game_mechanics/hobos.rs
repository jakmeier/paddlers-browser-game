// TODO [0.1.5] Move to better place, maybe a separate specification / balancing crate.

pub struct HoboLevel(usize);

impl HoboLevel {
    pub fn zero() -> Self {
        HoboLevel(0)
    }
    pub fn anarchist(player_karma: i64) -> Self {
        match player_karma {
            0..=19 => HoboLevel(0),
            20..=99 => HoboLevel(1),
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
