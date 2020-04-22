/// Defines API error codes to be sent over the network
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PadlApiError {
    PlayerNotCreated = 1,
}

impl std::error::Error for PadlApiError {}
impl std::fmt::Display for PadlApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PadlApiError::PlayerNotCreated => write!(f, "The player is not in the database."),
        }
    }
}

impl PadlApiError {
    pub fn try_from_num(i: u8) -> Option<Self> {
        match i {
            1 => Some(PadlApiError::PlayerNotCreated),
            _ => None,
        }
    }
}
