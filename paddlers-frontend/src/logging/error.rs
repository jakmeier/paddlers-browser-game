use std::{fmt};
use crate::prelude::*;
use crate::game::town::{TileType, TileIndex};

pub type PadlResult<R> = Result<R, PadlError>;

#[derive(Debug)]
pub struct PadlError {
    pub err: PadlErrorCode,
    pub (super) channel: ErrorChannel,
}

#[derive(Debug, Clone, Copy)]
pub (super) enum ErrorChannel {
    UserFacing,
    Technical
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
    BuildingFull(BuildingType),
    ForestTooSmall(usize),
    PathBlocked,
    // Dev only
    DevMsg(&'static str),
    MapOverflow(TileIndex),
    UnexpectedTileType(&'static str, TileType),
    RestAPI(String),
    EmptyGraphQLData(&'static str),
    StdWebGenericError(stdweb::web::error::Error),
    StdWebConversion(stdweb::private::ConversionError),
    JsonParseError(serde_json::error::Error),
}

impl fmt::Display for PadlErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PadlErrorCode::TestError =>
                write!(f, "This is only used for testing"),
            // User
            PadlErrorCode::BuildingFull(b) =>
                write!(f, "The {} is full.", b),
            PadlErrorCode::ForestTooSmall(amount) =>
                write!(f, "Missing {} forest flora size.", amount),
            PadlErrorCode::PathBlocked =>
                write!(f, "The path is blocked."),
            // Dev
            PadlErrorCode::DevMsg(msg) =>
                write!(f, "Dev Error Msg: {}", msg),
            PadlErrorCode::MapOverflow(i) =>
                write!(f, "Index is outside the map: {:?}", i),
            PadlErrorCode::UnexpectedTileType(expected, was) =>
                write!(f, "Unexpected tile type: Expected {} but was {:?}", expected, was),
            PadlErrorCode::RestAPI(msg) =>
                write!(f, "A REST API error occurred: {}", msg),
            PadlErrorCode::EmptyGraphQLData(data_set) =>
                write!(f, "GraphQL query result has no data for: {}", data_set),
            PadlErrorCode::StdWebGenericError(cause) =>
                write!(f, "A web error ocurred: {}", cause),
            PadlErrorCode::StdWebConversion(cause) =>
                write!(f, "A conversion error in the web std library occurred: {}", cause),
            PadlErrorCode::JsonParseError(cause) =>
                write!(f, "Error while parsing JSON data: {}", cause),
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

impl From<stdweb::web::error::Error> for PadlError {
    fn from(error: stdweb::web::error::Error) -> Self {
        PadlError::dev_err(PadlErrorCode::StdWebGenericError(error))
    }
}