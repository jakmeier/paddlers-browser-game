use std::{fmt};
use crate::prelude::*;
use crate::game::town::{TileType, TileIndex};

pub type PadlResult<R> = Result<R, PadlError>;

#[derive(Debug, Clone)]
pub struct PadlError {
    pub err: PadlErrorCode,
    pub cause: Option<Box<PadlError>>,
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
            cause: None,
            channel: chan,
        }
    }
    pub fn user_err<R>(err: PadlErrorCode) -> Result<R,PadlError> {
        Err(PadlError::new(err, ErrorChannel::UserFacing))
    }
    pub fn dev_err<R>(err: PadlErrorCode) -> Result<R,PadlError> {
        Err(PadlError::new(err, ErrorChannel::Technical))
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
        PadlError::user_err(self)
    }
    pub fn dev<R>(self) -> PadlResult<R> {
        PadlError::dev_err(self)
    }
}

#[derive(Debug, Clone)]
pub enum PadlErrorCode {
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
                write!(f, "A REST API error occured: {}", msg),
        }
    }
}