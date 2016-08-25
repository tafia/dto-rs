extern crate rustc_serialize;

use std::fmt;
use std::error::Error;
use rustc_serialize::{Encodable, Decodable};

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct AccessTokenDTO {
    pub app_id: String,
    pub scopes: String,
    pub access_token: String,
    pub token_type: TokenTypeDTO,
    pub expiration: i64,
}

impl DTO for AccessTokenDTO {}

#[derive(Debug, PartialEq, Eq, Copy, Clone, RustcDecodable, RustcEncodable)]
pub enum TokenTypeDTO {
    Bearer,
}

impl fmt::Display for TokenTypeDTO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DTO for TokenTypeDTO {}

pub trait DTO: Encodable + Decodable {}

pub trait FromDTO<D: DTO>: Sized {
    fn from_dto(dto: D) -> Result<Self, FromDTOError>;
}

#[derive(Debug)]
pub struct FromDTOError;

impl fmt::Display for FromDTOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for FromDTOError {
    fn description(&self) -> &str {
        "Something went wrong when converting the DTO to the required type"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
