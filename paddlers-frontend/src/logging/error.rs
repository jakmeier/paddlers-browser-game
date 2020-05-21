use crate::game::town::{TileIndex, TileType};
use crate::net::ajax::AjaxError;
use crate::prelude::*;
use crate::stdweb::unstable::TryInto;
use std::fmt;
use std::sync::mpsc::SendError;

pub type PadlResult<R> = Result<R, PadlError>;

#[derive(Debug)]
pub struct PadlError {
    pub err: PadlErrorCode,
    pub(super) channel: ErrorChannel,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum ErrorChannel {
    UserFacing,
    Technical,
}

impl PadlError {
    fn new(err: PadlErrorCode, chan: ErrorChannel) -> PadlError {
        PadlError {
            err: err,
            channel: chan,
        }
    }
    pub fn user_err(err: PadlErrorCode) -> PadlError {
        PadlError::new(err, ErrorChannel::UserFacing)
    }
    pub fn dev_err(err: PadlErrorCode) -> PadlError {
        PadlError::new(err, ErrorChannel::Technical)
    }
}

impl std::error::Error for PadlError {}
impl fmt::Display for PadlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl PadlErrorCode {
    pub fn usr<R>(self) -> PadlResult<R> {
        Err(PadlError::user_err(self))
    }
    pub fn dev<R>(self) -> PadlResult<R> {
        Err(PadlError::dev_err(self))
    }
}

#[derive(Debug)]
pub enum PadlErrorCode {
    #[allow(dead_code)]
    TestError,
    // User
    BuildingFull(Option<BuildingType>),
    ForestTooSmall(usize),
    NotEnoughResources,
    NotEnoughSupply,
    NotEnoughMana,
    NotEnoughKarma,
    NotEnoughUnits,
    NotReadyYet,
    PathBlocked,
    NoNetwork,
    NestEmpty,
    // Dev only
    DevMsg(&'static str),
    MapOverflow(TileIndex),
    NoStateForTile(TileIndex),
    UnexpectedTileType(&'static str, TileType),
    MissingComponent(&'static str),
    EcsError(&'static str),
    SpecsError(specs::error::Error),
    EventPoolSend(SendError<GameEvent>),
    RestAPI(String),
    EmptyGraphQLData(&'static str),
    InvalidGraphQLData(&'static str),
    UnknownNetObj(crate::game::components::NetObj),
    StdWebGenericError(stdweb::web::error::Error),
    StdWebConversion(stdweb::private::ConversionError),
    StdWebSecurityError(stdweb::web::error::SecurityError),
    InvalidDom(&'static str),
    QuicksilverError(String),
    PanesError(String),
    JsonParseError(serde_json::error::Error),
    UrlParseError(String),
    NoDataFromBrowser(&'static str),
    BrowserError(String),
    DialogueEmpty,
    UserNotInDB,
    AuthorizationRequired,
    DataForInactiveTownReceived(&'static str),
}

impl fmt::Display for PadlErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PadlErrorCode::TestError => write!(f, "This is only used for testing"),
            // User
            PadlErrorCode::BuildingFull(Some(b)) => write!(f, "The {} is full.", b),
            PadlErrorCode::BuildingFull(None) => write!(f, "Building is full."),
            PadlErrorCode::ForestTooSmall(amount) => {
                write!(f, "Missing {} forest flora size.", amount)
            }
            PadlErrorCode::NotReadyYet => write!(f, "Patience! This is not ready, yet."),
            PadlErrorCode::NotEnoughResources => write!(f, "Need more resources."),
            PadlErrorCode::NotEnoughSupply => write!(f, "Requires more supplies."),
            PadlErrorCode::NotEnoughMana => write!(f, "Not enough mana."),
            PadlErrorCode::NotEnoughKarma => write!(f, "Not enough karma."),
            PadlErrorCode::NotEnoughUnits => write!(f, "Require more units."),
            PadlErrorCode::PathBlocked => write!(f, "The path is blocked."),
            PadlErrorCode::NoNetwork => write!(f, "Connection to server dropped."),
            PadlErrorCode::NestEmpty => write!(f, "Nobody around to invite."),
            // Dev
            PadlErrorCode::DevMsg(msg) => write!(f, "Dev Error Msg: {}", msg),
            PadlErrorCode::MapOverflow(i) => write!(f, "Index is outside the map: {:?}", i),
            PadlErrorCode::NoStateForTile(i) => write!(f, "No state found for tile: {:?}", i),
            PadlErrorCode::UnexpectedTileType(expected, was) => write!(
                f,
                "Unexpected tile type: Expected {} but was {:?}",
                expected, was
            ),
            PadlErrorCode::MissingComponent(component) => {
                write!(f, "Entity does not have required component: {}", component)
            }
            PadlErrorCode::EcsError(component) => write!(f, "ECS error: {}", component),
            PadlErrorCode::SpecsError(component) => write!(f, "Specs error: {}", component),
            PadlErrorCode::EventPoolSend(e) => write!(f, "EventPool send error: {}", e),
            PadlErrorCode::RestAPI(msg) => write!(f, "A REST API error occurred: {}", msg),
            PadlErrorCode::EmptyGraphQLData(data_set) => {
                write!(f, "GraphQL query result has no data for: {}", data_set)
            }
            PadlErrorCode::InvalidGraphQLData(reason) => {
                write!(f, "GraphQL query result has invalid data: {}", reason)
            }
            PadlErrorCode::UnknownNetObj(key) => {
                write!(f, "GraphQL query result has unknown key: {:?}", key)
            }
            PadlErrorCode::StdWebGenericError(cause) => write!(f, "A web error ocurred: {}", cause),
            PadlErrorCode::StdWebConversion(cause) => write!(
                f,
                "A conversion error in the web std library occurred: {}",
                cause
            ),
            PadlErrorCode::StdWebSecurityError(cause) => write!(
                f,
                "A security error in the web std library occurred: {}",
                cause
            ),
            PadlErrorCode::InvalidDom(cause) => write!(f, "DOM error: {}", cause),
            PadlErrorCode::QuicksilverError(cause) => write!(f, "Quicksilver error: {}", cause),
            PadlErrorCode::PanesError(cause) => write!(f, "Panes error: {}", cause),
            PadlErrorCode::JsonParseError(cause) => {
                write!(f, "Error while parsing JSON data: {}", cause)
            }
            PadlErrorCode::UrlParseError(cause) => {
                write!(f, "Error while parsing browser URL: {}", cause)
            }
            PadlErrorCode::NoDataFromBrowser(data) => {
                write!(f, "Could not read data from browser: {}", data)
            }
            PadlErrorCode::BrowserError(s) => write!(f, "Unexpected browser error: {}", s),
            PadlErrorCode::DialogueEmpty => write!(f, "No scene loaded in dialogue"),
            PadlErrorCode::UserNotInDB => {
                write!(f, "The user logged in is not present in the game database.")
            }
            PadlErrorCode::AuthorizationRequired => {
                write!(f, "The requested resource permits authorized access only.")
            }
            PadlErrorCode::DataForInactiveTownReceived(data) => {
                write!(f, "Received data {} for town that is not active.", data)
            }
        }
    }
}

impl From<stdweb::private::ConversionError> for PadlError {
    fn from(error: stdweb::private::ConversionError) -> Self {
        PadlError::dev_err(PadlErrorCode::StdWebConversion(error))
    }
}

impl From<serde_json::error::Error> for PadlError {
    fn from(error: serde_json::error::Error) -> Self {
        PadlError::dev_err(PadlErrorCode::JsonParseError(error))
    }
}

impl From<url::ParseError> for PadlError {
    fn from(error: url::ParseError) -> Self {
        PadlError::dev_err(PadlErrorCode::UrlParseError(format!("{}", error)))
    }
}

impl From<stdweb::web::error::Error> for PadlError {
    fn from(error: stdweb::web::error::Error) -> Self {
        PadlError::dev_err(PadlErrorCode::StdWebGenericError(error))
    }
}
impl From<stdweb::Value> for PadlError {
    fn from(val: stdweb::Value) -> Self {
        let s: String = js! { return String(@{val}); }
            .try_into()
            .unwrap_or("Reading Browser Error Value failed".to_owned());
        PadlError::dev_err(PadlErrorCode::BrowserError(s))
    }
}
impl From<stdweb::web::error::SecurityError> for PadlError {
    fn from(error: stdweb::web::error::SecurityError) -> Self {
        PadlError::dev_err(PadlErrorCode::StdWebSecurityError(error))
    }
}
impl From<AjaxError> for PadlError {
    fn from(ajax: AjaxError) -> Self {
        if let Some(e) = ajax.padl_error {
            PadlError::dev_err(e)
        } else {
            PadlError::dev_err(PadlErrorCode::BrowserError(ajax.description))
        }
    }
}
impl From<quicksilver::Error> for PadlError {
    fn from(error: quicksilver::Error) -> Self {
        PadlError::dev_err(PadlErrorCode::QuicksilverError(error.to_string()))
    }
}
impl From<panes::PanesError> for PadlError {
    fn from(error: panes::PanesError) -> Self {
        PadlError::dev_err(PadlErrorCode::PanesError(error.to_string()))
    }
}
impl From<SendError<GameEvent>> for PadlError {
    fn from(error: SendError<GameEvent>) -> Self {
        PadlError::dev_err(PadlErrorCode::EventPoolSend(error))
    }
}
impl From<specs::error::Error> for PadlError {
    fn from(error: specs::error::Error) -> Self {
        PadlError::dev_err(PadlErrorCode::SpecsError(error))
    }
}
impl From<&'static str> for PadlError {
    fn from(msg: &'static str) -> Self {
        PadlError::dev_err(PadlErrorCode::DevMsg(msg))
    }
}

impl From<paddlers_shared_lib::game_mechanics::town::TownError> for PadlError {
    fn from(error: paddlers_shared_lib::game_mechanics::town::TownError) -> Self {
        use paddlers_shared_lib::game_mechanics::town::TownError;
        match error {
            TownError::BuildingFull => PadlError::user_err(PadlErrorCode::BuildingFull(None)),
            TownError::NotEnoughSupply => PadlError::user_err(PadlErrorCode::NotEnoughSupply),
            TownError::InvalidState(s) => PadlError::dev_err(PadlErrorCode::DevMsg(s)),
        }
    }
}
