#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum CivilizationPerk {
    /// Allows to build (single) nests which can hold hobos
    NestBuilding,
    /// Allows to build triple nests which can hold hobos
    TripleNestBuilding,
    /// Allows to send invitation to foreign hobos
    Invitation,
    /// Allows to convert visiting hobos and thereby increase the worker count
    Conversion,
}

// i32 is just the largest common denominator for current gql endpoint libraries used. Will have to change once > 31 perks are implemented.
pub type SerializedCivPerks = i32;

#[derive(Clone, Copy)]
pub struct CivilizationPerks {
    bitflags: u32,
}

impl CivilizationPerks {
    pub fn has(&self, p: CivilizationPerk) -> bool {
        let index = p as usize;
        (self.bitflags >> index) & 1 == 1
    }
    pub fn set(&mut self, p: CivilizationPerk) {
        let index = p as usize;
        self.bitflags |= 1 << index;
    }
    pub fn new(bitflags: u32) -> Self {
        Self { bitflags }
    }
    pub fn encode(self) -> SerializedCivPerks {
        debug_assert!(self.bitflags < std::i32::MAX as u32);
        self.bitflags as i32
    }
    pub fn decode(encoded: SerializedCivPerks) -> Self {
        debug_assert!(encoded >= 0);
        Self {
            bitflags: encoded as u32,
        }
    }
}

impl std::fmt::Debug for CivilizationPerks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..32 {
            if self.bitflags >> i & 1 == 1 {
                write!(f, "{:?},", CivilizationPerks::decode(i as i32))?;
            }
        }
        write!(f, "]")
    }
}

#[cfg(feature = "sql_db")]
impl crate::prelude::Player {
    pub fn civilization_perks(&self) -> CivilizationPerks {
        CivilizationPerks::new(self.civ_perks as u32)
    }
}
